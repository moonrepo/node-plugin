use extism_pdk::*;
use node_common::{BinField, PackageJson};
use proto_pdk::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
    let name = if input.id.contains("yarn") {
        "yarn"
    } else if input.id.contains("pnpm") {
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
                    bin_path = Some(PathBuf::from(bin));
                }
                BinField::Object(map) => {
                    if let Some(bin) = map.get(get_package_name()) {
                        bin_path = Some(PathBuf::from(bin));
                    }
                }
            };
        }

        if bin_path.is_none() {
            if let Some(main_field) = package_json.main {
                bin_path = Some(PathBuf::from(main_field));
            }
        }
    }

    if bin_path.is_none() {
        bin_path = Some(PathBuf::from("bin").join(if is_pnpm() {
            "pnpm.cjs"
        } else {
            get_package_name()
        }));
    }

    Ok(Json(LocateBinsOutput {
        bin_path,
        globals_lookup_dirs: vec!["$PROTO_ROOT/tools/node/globals/bin".into()],
    }))
}
