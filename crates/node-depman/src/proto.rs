use extism_pdk::*;
use node_common::{commands, BinField, NodeDistVersion, PackageJson};
use proto_pdk::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
    fn host_log(input: Json<HostLogInput>);
}

#[derive(PartialEq)]
enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
}

impl PackageManager {
    pub fn detect() -> PackageManager {
        let id = get_tool_id();

        if id.to_lowercase().contains("yarn") {
            PackageManager::Yarn
        } else if id.to_lowercase().contains("pnpm") {
            PackageManager::Pnpm
        } else {
            PackageManager::Npm
        }
    }

    pub fn get_package_name(&self, version: &str) -> String {
        if self.is_yarn_berry(version) {
            "@yarnpkg/cli-dist".into()
        } else {
            self.to_string()
        }
    }

    pub fn is_yarn_classic(&self, version: &str) -> bool {
        self == &PackageManager::Yarn
            && (version.starts_with('1') || version == "legacy" || version == "classic")
    }

    pub fn is_yarn_berry(&self, version: &str) -> bool {
        self == &PackageManager::Yarn
            && (!version.starts_with('1') || version == "berry" || version == "latest")
    }
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageManager::Npm => write!(f, "npm"),
            PackageManager::Pnpm => write!(f, "pnpm"),
            PackageManager::Yarn => write!(f, "yarn"),
        }
    }
}

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    let manager = PackageManager::detect();

    Ok(Json(ToolMetadataOutput {
        name: manager.to_string(),
        type_of: PluginType::DependencyManager,
        env_vars: vec!["PROTO_NODE_VERSION".into(), "PROTO_INSTALL_GLOBAL".into()],
        default_version: if manager == PackageManager::Npm {
            Some("bundled".into())
        } else {
            None
        },
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let version = &input.context.version;
    let manager = PackageManager::detect();

    if version == "canary" {
        return err!(PluginError::UnsupportedCanary {
            tool: manager.to_string()
        }
        .into());
    }

    let package_name = manager.get_package_name(version);

    // Derive values based on package manager
    let archive_prefix = if manager.is_yarn_classic(version) {
        format!("yarn-v{version}")
    } else {
        "package".into()
    };

    let package_without_scope = if package_name.contains('/') {
        package_name.split('/').nth(1).unwrap()
    } else {
        &package_name
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(archive_prefix),
        download_url: format!(
            "https://registry.npmjs.org/{package_name}/-/{package_without_scope}-{version}.tgz",
        ),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_bins(Json(input): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    let mut bin_path = None;
    let package_path = input.context.tool_dir.join("package.json");
    let manager = PackageManager::detect();
    let manager_name = manager.to_string();

    // Extract the binary from the `package.json`
    if package_path.exists() {
        if let Ok(package_json) = json::from_slice::<PackageJson>(&fs::read(package_path)?) {
            if let Some(bin_field) = package_json.bin {
                match bin_field {
                    BinField::String(bin) => {
                        bin_path = Some(bin);
                    }
                    BinField::Object(map) => {
                        if let Some(bin) = map.get(&manager_name) {
                            bin_path = Some(bin.to_owned());
                        }
                    }
                };
            }

            if bin_path.is_none() {
                if let Some(main_field) = package_json.main {
                    bin_path = Some(main_field);
                }
            }
        }
    }

    if bin_path.is_none() {
        bin_path = Some(format!(
            "bin/{}",
            if manager == PackageManager::Pnpm {
                "pnpm.cjs".to_owned()
            } else {
                manager_name
            }
        ));
    }

    Ok(Json(LocateBinsOutput {
        bin_path: bin_path.map(PathBuf::from),
        fallback_last_globals_dir: true,
        globals_lookup_dirs: vec!["$PROTO_ROOT/tools/node/globals/bin".into()],
        ..LocateBinsOutput::default()
    }))
}

#[derive(Deserialize)]
struct RegistryVersion {
    version: String, // No v prefix
}

#[derive(Deserialize)]
struct RegistryResponse {
    #[serde(rename = "dist-tags")]
    dist_tags: HashMap<String, String>,
    versions: HashMap<String, RegistryVersion>,
}

#[plugin_fn]
pub fn load_versions(Json(input): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let mut output = LoadVersionsOutput::default();
    let manager = PackageManager::detect();
    let package_name = manager.get_package_name(&input.initial);

    let mut map_output = |res: RegistryResponse| -> Result<(), Error> {
        for item in res.versions.values() {
            output.versions.push(Version::parse(&item.version)?);
        }

        // Dist tags always includes latest
        for (alias, version) in res.dist_tags {
            let version = Version::parse(&version)?;

            if alias == "latest" && output.latest.is_none() {
                output.latest = Some(version.clone());
            }

            output.aliases.entry(alias).or_insert(version);
        }

        Ok(())
    };

    map_output(fetch_url_with_cache(format!(
        "https://registry.npmjs.org/{}/",
        package_name
    ))?)?;

    // Yarn is managed by 2 different packages, so we need to request versions from both of them!
    if manager.is_yarn_berry(&input.initial) {
        map_output(fetch_url_with_cache("https://registry.npmjs.org/yarn/")?)?;
    } else if manager.is_yarn_classic(&input.initial) {
        map_output(fetch_url_with_cache(
            "https://registry.npmjs.org/@yarnpkg/cli-dist/",
        )?)?;
    }

    output
        .aliases
        .insert("latest".into(), output.latest.clone().unwrap());

    Ok(Json(output))
}

#[plugin_fn]
pub fn resolve_version(
    Json(input): Json<ResolveVersionInput>,
) -> FnResult<Json<ResolveVersionOutput>> {
    let manager = PackageManager::detect();
    let mut output = ResolveVersionOutput::default();

    match manager {
        PackageManager::Npm => {
            // When the alias "bundled" is provided, we should install the npm
            // version that comes bundled with the current Node.js version.
            if input.initial == "bundled" {
                let response: Vec<NodeDistVersion> =
                    fetch_url_with_cache("https://nodejs.org/download/release/index.json")?;
                let mut found_version = false;

                // Infer from proto's environment variable
                if let Some(node_version) = input.context.env_vars.get("PROTO_NODE_VERSION") {
                    for node_release in &response {
                        // Theirs starts with v, ours does not
                        if &node_release.version[1..] == node_version {
                            output.version = node_release.npm.clone();
                            found_version = true;
                            break;
                        }
                    }
                }

                // Otherwise call the current `node` binary and infer from that
                if !found_version {
                    let node_version = exec_command!("node", ["--version"]);
                    let node_version = node_version.stdout.trim();

                    for node_release in &response {
                        // Both start with v
                        if node_release.version == node_version {
                            output.version = node_release.npm.clone();
                            found_version = true;
                            break;
                        }
                    }
                }

                if !found_version {
                    host_log!(
                        "Could not find a bundled npm version for Node.js, falling back to latest"
                    );

                    output.candidate = Some("latest".into());
                }
            }
        }

        PackageManager::Yarn => {
            // Latest currently resolves to a v4-rc version...
            if input.initial == "berry" || input.initial == "latest" {
                output.candidate = Some("~3".into());
            } else if input.initial == "legacy" || input.initial == "classic" {
                output.candidate = Some("~1".into());
            }
        }

        _ => {}
    };

    Ok(Json(output))
}

#[plugin_fn]
pub fn create_shims(Json(_): Json<CreateShimsInput>) -> FnResult<Json<CreateShimsOutput>> {
    let env = get_proto_environment()?;
    let manager = PackageManager::detect();
    let mut global_shims = HashMap::<String, ShimConfig>::new();
    let mut local_shims = HashMap::<String, ShimConfig>::new();

    match manager {
        PackageManager::Npm => {
            local_shims.insert(
                "npm".into(),
                ShimConfig::local_with_parent("bin/npm-cli.js", "node"),
            );

            // node-gyp
            global_shims.insert(
                "node-gyp".into(),
                ShimConfig::global_with_alt_bin(if env.os == HostOS::Windows {
                    "bin/node-gyp-bin/node-gyp.cmd"
                } else {
                    "bin/node-gyp-bin/node-gyp"
                }),
            );
        }
        PackageManager::Pnpm => {
            local_shims.insert(
                "pnpm".into(),
                ShimConfig::local_with_parent("bin/pnpm.cjs", "node"),
            );

            // pnpx
            global_shims.insert("pnpx".into(), ShimConfig::global_with_sub_command("dlx"));
        }
        PackageManager::Yarn => {
            local_shims.insert(
                "yarn".into(),
                ShimConfig::local_with_parent("bin/yarn.js", "node"),
            );

            // yarnpkg
            global_shims.insert("yarnpkg".into(), ShimConfig::default());
        }
    };

    Ok(Json(CreateShimsOutput {
        primary: Some(ShimConfig {
            parent_bin: Some("node".into()),
            ..ShimConfig::default()
        }),
        global_shims,
        local_shims,
        ..CreateShimsOutput::default()
    }))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec!["package.json".into()],
    }))
}

#[plugin_fn]
pub fn parse_version_file(
    Json(input): Json<ParseVersionFileInput>,
) -> FnResult<Json<ParseVersionFileOutput>> {
    let mut version = None;

    if input.file == "package.json" {
        if let Ok(package_json) = json::from_str::<PackageJson>(&input.content) {
            if let Some(pm) = package_json.package_manager {
                let mut parts = pm.split('@');
                let name = parts.next().unwrap_or_default();

                if name == PackageManager::detect().to_string() {
                    version = Some(parts.next().unwrap_or("latest").to_owned());
                }
            }
        }
    }

    Ok(Json(ParseVersionFileOutput { version }))
}

#[plugin_fn]
pub fn install_global(
    Json(input): Json<InstallGlobalInput>,
) -> FnResult<Json<InstallGlobalOutput>> {
    let result = exec_command!(commands::install_global(
        &input.dependency,
        &input.globals_dir.real_path(),
    ));

    Ok(Json(InstallGlobalOutput::from_exec_command(result)))
}

#[plugin_fn]
pub fn uninstall_global(
    Json(input): Json<UninstallGlobalInput>,
) -> FnResult<Json<UninstallGlobalOutput>> {
    let result = exec_command!(commands::uninstall_global(
        &input.dependency,
        &input.globals_dir.real_path(),
    ));

    Ok(Json(UninstallGlobalOutput::from_exec_command(result)))
}

#[plugin_fn]
pub fn pre_run(Json(input): Json<RunHook>) -> FnResult<()> {
    let args = &input.passthrough_args;
    let user_config = get_proto_user_config()?;

    if args.len() < 3
        || input.context.env_vars.get("PROTO_INSTALL_GLOBAL").is_some()
        || !user_config.node_intercept_globals
    {
        return Ok(());
    }

    let manager = PackageManager::detect();
    let mut is_install_command = false;
    let mut is_global = false;

    // npm install -g <dep>
    // pnpm add -g <dep>
    if manager == PackageManager::Npm || manager == PackageManager::Pnpm {
        is_install_command = args[0] == "install" || args[0] == "i" || args[0] == "add";

        for arg in args {
            if arg == "--global" || arg == "-g" || arg == "--location=global" {
                is_global = true;
                break;
            }
        }
    }

    // yarn global add <dep>
    if manager == PackageManager::Yarn {
        is_global = args[0] == "global";
        is_install_command = args[1] == "add";
    }

    if is_install_command && is_global {
        return err!(
            "Global binaries must be installed with `proto install-global {}`!\nLearn more: {}\n\nOpt-out of this functionality with `{}`.",
            manager.to_string(),
            "https://moonrepo.dev/docs/proto/faq#how-can-i-install-a-global-binary-for-a-language",
            "node-intercept-globals = false",
        );
    }

    Ok(())
}
