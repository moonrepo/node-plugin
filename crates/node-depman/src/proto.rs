use extism_pdk::*;
use node_common::{BinField, NodeDistVersion, PackageJson};
use proto_pdk::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs;

#[host_fn]
extern "ExtismHost" {
    fn trace(input: Json<TraceInput>);
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

#[derive(PartialEq)]
enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
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

fn get_package_manager(env: &Environment) -> PackageManager {
    if env.id.to_lowercase().contains("yarn") {
        PackageManager::Yarn
    } else if env.id.to_lowercase().contains("pnpm") {
        PackageManager::Pnpm
    } else {
        PackageManager::Npm
    }
}

#[plugin_fn]
pub fn register_tool(Json(input): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    let manager = get_package_manager(&input.env);

    Ok(Json(ToolMetadataOutput {
        name: manager.to_string(),
        type_of: PluginType::DependencyManager,
        env_vars: vec![],
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let manager = get_package_manager(&input.env);
    let version = input.env.version;

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(if manager == PackageManager::Yarn {
            format!("yarn-v{version}")
        } else {
            "package".into()
        }),
        download_url: format!(
            "https://registry.npmjs.org/{name}/-/{name}-{version}.tgz",
            name = manager.to_string()
        ),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_bins(Json(input): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    let mut bin_path = None;
    let package_path = input.tool_dir.join("package.json");
    let manager = get_package_manager(&input.env);

    // Extract the binary from the `package.json`
    if package_path.exists() {
        let package_json: PackageJson = json::from_slice(&fs::read(package_path)?)?;

        if let Some(bin_field) = package_json.bin {
            match bin_field {
                BinField::String(bin) => {
                    bin_path = Some(bin);
                }
                BinField::Object(map) => {
                    if let Some(bin) = map.get(&manager.to_string()) {
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

    if bin_path.is_none() {
        bin_path = Some(format!(
            "bin/{}",
            if manager == PackageManager::Pnpm {
                "pnpm.cjs".to_owned()
            } else {
                manager.to_string()
            }
        ));
    }

    Ok(Json(LocateBinsOutput {
        bin_path,
        fallback_last_globals_dir: true,
        globals_lookup_dirs: vec!["$PROTO_ROOT/tools/node/globals/bin".into()],
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
    let manager = get_package_manager(&input.env);
    let response: RegistryResponse =
        fetch_url_with_cache(format!("https://registry.npmjs.org/{}/", manager))?;

    for item in response.versions.values() {
        output.versions.push(Version::parse(&item.version)?);
    }

    // Dist tags always includes latest
    for (alias, version) in response.dist_tags {
        let version = Version::parse(&version)?;

        if alias == "latest" {
            output.latest = Some(version.clone());
        }

        output.aliases.insert(alias, version);
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn resolve_version(
    Json(input): Json<ResolveVersionInput>,
) -> FnResult<Json<ResolveVersionOutput>> {
    let manager = get_package_manager(&input.env);
    let mut output = ResolveVersionOutput::default();

    match manager {
        // When the alias "bundled" is provided, we should install the npm
        // version that comes bundled with the current Node.js version.
        PackageManager::Npm => {
            if input.initial == "bundled" {
                let node_version =
                    unsafe { exec_command(Json(ExecCommandInput::new("node", ["--version"])))?.0 };
                let node_version = node_version.stdout.trim(); // Has v prefix

                let response: Vec<NodeDistVersion> =
                    fetch_url_with_cache("https://nodejs.org/dist/index.json")?;

                for node_release in response {
                    if node_release.version == node_version {
                        output.candidate = node_release.npm;
                        break;
                    }
                }

                if output.candidate.is_none() {
                    unsafe {
                        trace(Json(
                            format!("Could not find a bundled npm version for Node.js {}, falling back to latest", node_version).into()
                        ))?;
                    }

                    output.candidate = Some("latest".into());
                }
            }
        }

        // Yarn is installed through npm, but only v1 exists in the npm registry,
        // even if a consumer is using Yarn 2/3. https://www.npmjs.com/package/yarn
        // Yarn >= 2 works differently than normal packages, as their runtime code
        // is stored *within* the repository, and the v1 package detects it.
        // Because of this, we need to always install the v1 package!
        PackageManager::Yarn => {
            if !input.initial.starts_with('1') {
                unsafe {
                    trace(Json(
                        "Found Yarn v2+, installing latest v1 from registry for compatibility"
                            .into(),
                    ))?;
                }

                output.candidate = Some("1.22.19".into())
            }
        }

        _ => {}
    };

    Ok(Json(output))
}

#[plugin_fn]
pub fn create_shims(Json(input): Json<CreateShimsInput>) -> FnResult<Json<CreateShimsOutput>> {
    let manager = get_package_manager(&input.env);
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
                ShimConfig::global_with_alt_bin(if input.env.os == HostOS::Windows {
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
    let manager = get_package_manager(&input.env);

    if input.file == "package.json" {
        let package_json: PackageJson = json::from_str(&input.content)?;
        let package_name = manager.to_string();

        if let Some(manager) = package_json.package_manager {
            let mut parts = manager.split('@');
            let name = parts.next().unwrap_or_default();

            if name == package_name {
                version = Some(parts.next().unwrap_or("latest").to_owned());
            }
        }

        if version.is_none() {
            if let Some(engines) = package_json.engines {
                if let Some(constraint) = engines.get(&package_name) {
                    version = Some(constraint.to_owned());
                }
            }
        }
    }

    Ok(Json(ParseVersionFileOutput { version }))
}
