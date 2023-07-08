use extism_pdk::*;
use node_common::{BinField, NodeDistVersion, PackageJson};
use proto_pdk::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

static mut NAME: &str = "npm";
static mut BIN: &str = "npm";

fn is_pnpm() -> bool {
    unsafe { BIN == "pnpm" }
}

fn is_yarn() -> bool {
    unsafe { BIN == "yarn" }
}

fn get_package_name() -> &'static str {
    unsafe { NAME }
}

#[plugin_fn]
pub fn register_tool(Json(input): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    let name = if input.id.to_lowercase().contains("yarn") {
        "yarn"
    } else if input.id.to_lowercase().contains("pnpm") {
        "pnpm"
    } else {
        "npm"
    };

    // This is safe since this function is only called once!
    unsafe {
        NAME = name;
        BIN = name;
    }

    Ok(Json(ToolMetadataOutput {
        name: name.into(),
        type_of: PluginType::DependencyManager,
        env_vars: vec![],
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let name = get_package_name();
    let version = input.env.version;

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(if is_yarn() {
            format!("yarn-v{version}")
        } else {
            "package".into()
        }),
        download_url: format!("https://registry.npmjs.org/{name}/-/{name}-{version}.tgz"),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_bins(Json(input): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    let mut bin_path = None;
    let package_path = input.tool_dir.join("package.json");

    // Extract the binary from the `package.json`
    if package_path.exists() {
        let package_json: PackageJson = json::from_slice(&fs::read(package_path)?)?;

        if let Some(bin_field) = package_json.bin {
            match bin_field {
                BinField::String(bin) => {
                    bin_path = Some(bin);
                }
                BinField::Object(map) => {
                    if let Some(bin) = map.get(get_package_name()) {
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
            if is_pnpm() {
                "pnpm.cjs"
            } else {
                get_package_name()
            }
        ));
    }

    Ok(Json(LocateBinsOutput {
        bin_path,
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
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let mut output = LoadVersionsOutput::default();
    let response: RegistryResponse = fetch_url(format!(
        "https://registry.npmjs.org/{}/",
        get_package_name()
    ))?;

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
    let mut output = ResolveVersionOutput::default();

    match get_package_name() {
        // When the alias "bundled" is provided, we should install the npm
        // version that comes bundled with the current Node.js version.
        "npm" => {
            if input.initial == "bundled" {
                let node_version =
                    String::from_utf8(Command::new("node").arg("--version").output()?.stdout)?;
                let node_version = node_version.trim(); // Has v prefix

                let response: Vec<NodeDistVersion> =
                    fetch_url("https://nodejs.org/dist/index.json")?;

                for node_release in response {
                    if node_release.version == node_version {
                        output.candidate = node_release.npm;
                        break;
                    }
                }

                if output.candidate.is_none() {
                    // debug!("Could not find a bundled npm version for Node.js {}, falling back to latest", node_version);

                    output.candidate = Some("latest".into());
                }
            }
        }

        // Yarn is installed through npm, but only v1 exists in the npm registry,
        // even if a consumer is using Yarn 2/3. https://www.npmjs.com/package/yarn
        // Yarn >= 2 works differently than normal packages, as their runtime code
        // is stored *within* the repository, and the v1 package detects it.
        // Because of this, we need to always install the v1 package!
        "yarn" => {
            if !input.initial.starts_with('1') {
                // debug!("Found Yarn v2+, installing latest v1 from registry for compatibility");

                output.candidate = Some("1.22.19".into())
            }
        }

        _ => {}
    };

    Ok(Json(output))
}

#[plugin_fn]
pub fn create_shims(Json(input): Json<CreateShimsInput>) -> FnResult<Json<CreateShimsOutput>> {
    let mut global_shims = HashMap::<String, ShimConfig>::new();
    let mut local_shims = HashMap::<String, ShimConfig>::new();

    match get_package_name() {
        "npm" => {
            local_shims.insert(
                "npm".into(),
                ShimConfig {
                    bin_path: Some("bin/npm-cli.js".into()),
                    parent_bin: Some("node".into()),
                    ..ShimConfig::default()
                },
            );

            // node-gyp
            global_shims.insert(
                "node-gyp".into(),
                ShimConfig {
                    bin_path: Some(if input.env.os == HostOS::Windows {
                        "bin/node-gyp-bin/node-gyp.cmd".into()
                    } else {
                        "bin/node-gyp-bin/node-gyp".into()
                    }),
                    ..ShimConfig::default()
                },
            );
        }
        "pnpm" => {
            local_shims.insert(
                "pnpm".into(),
                ShimConfig {
                    bin_path: Some("bin/pnpm.cjs".into()),
                    parent_bin: Some("node".into()),
                    ..ShimConfig::default()
                },
            );

            // pnpx
            global_shims.insert(
                "pnpx".into(),
                ShimConfig {
                    before_args: Some("dlx".into()),
                    ..ShimConfig::default()
                },
            );
        }
        "yarn" => {
            local_shims.insert(
                "yarn".into(),
                ShimConfig {
                    bin_path: Some("bin/yarn.js".into()),
                    parent_bin: Some("node".into()),
                    ..ShimConfig::default()
                },
            );

            // yarnpkg
            global_shims.insert("yarnpkg".into(), ShimConfig::default());
        }
        _ => {}
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
    Json(input): Json<ParseVersionInput>,
) -> FnResult<Json<ParseVersionOutput>> {
    let mut version = None;
    let package_name = get_package_name();

    if input.file == "package.json" {
        let package_json: PackageJson = json::from_str(&input.content)?;

        if let Some(manager) = package_json.package_manager {
            let mut parts = manager.split('@');
            let name = parts.next().unwrap_or_default();

            if name == package_name {
                version = Some(parts.next().unwrap_or("latest").to_owned());
            }
        }

        if version.is_none() {
            if let Some(engines) = package_json.engines {
                if let Some(constraint) = engines.get(package_name) {
                    version = Some(constraint.to_owned());
                }
            }
        }
    }

    Ok(Json(ParseVersionOutput { version }))
}
