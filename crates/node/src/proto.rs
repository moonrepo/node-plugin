use extism_pdk::*;
use node_common::{NodeDistLTS, NodeDistVersion, PackageJson};
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "Node.js";
static BIN: &str = "node";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        env_vars: vec!["NODE_OPTIONS".into(), "NODE_PATH".into()],
    }))
}

fn map_arch(os: HostOS, arch: HostArch) -> Result<String, PluginError> {
    let arch = match arch {
        HostArch::Arm => "armv7l".into(),
        HostArch::Arm64 => "arm64".into(),
        HostArch::Powerpc64 => {
            if os == HostOS::Linux {
                "ppc64le".into()
            } else {
                "ppc64".into()
            }
        }
        HostArch::S390x => "s390x".into(),
        HostArch::X64 => "x64".into(),
        HostArch::X86 => "x86".into(),
        other => {
            return Err(PluginError::UnsupportedArchitecture {
                tool: NAME.into(),
                arch: other.to_string(),
            });
        }
    };

    Ok(arch)
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let version = input.env.version;
    let arch = map_arch(input.env.os, input.env.arch)?;

    let prefix = match input.env.os {
        HostOS::Linux => format!("node-v{version}-linux-{arch}"),
        HostOS::MacOS => {
            let parsed_version = Version::parse(&version)?;

            // Arm64 support was added after v16, but M1/M2 machines can
            // run x64 binaries via Rosetta. This is a compat hack!
            if input.env.arch == HostArch::Arm64 && parsed_version.major < 16 {
                format!("node-v{version}-darwin-x64")
            } else {
                format!("node-v{version}-darwin-{arch}")
            }
        }
        HostOS::Windows => format!("node-v{version}-win-{arch}"),
        other => {
            return Err(PluginError::UnsupportedPlatform {
                tool: NAME.into(),
                platform: other.to_string(),
            })?;
        }
    };

    let filename = if input.env.os == HostOS::Windows {
        format!("{prefix}.zip")
    } else {
        format!("{prefix}.tar.xz")
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(prefix),
        download_url: format!("https://nodejs.org/dist/v{version}/{filename}"),
        download_name: Some(filename),
        checksum_url: Some(format!("https://nodejs.org/dist/v{version}/SHASUMS256.txt")),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_bins(Json(input): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    Ok(Json(LocateBinsOutput {
        bin_path: Some(if input.env.os == HostOS::Windows {
            format!("{}.exe", BIN)
        } else {
            BIN.to_owned()
        }),
        fallback_last_globals_dir: true,
        globals_lookup_dirs: vec!["$PROTO_ROOT/tools/node/globals/bin".into()],
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let mut output = LoadVersionsOutput::default();
    let response: Vec<NodeDistVersion> =
        fetch_url_with_cache("https://nodejs.org/dist/index.json")?;

    for (index, item) in response.iter().enumerate() {
        let version = Version::parse(&item.version[1..])?;

        // First item is always the latest
        if index == 0 {
            output.latest = Some(version.clone());
        }

        if let NodeDistLTS::Name(alias) = &item.lts {
            let alias = alias.to_lowercase();

            // The first encounter of an lts is the latest stable
            if !output.aliases.contains_key("stable") {
                output.aliases.insert("stable".into(), version.clone());
            }

            // The first encounter of an lts is the latest version for that alias
            if !output.aliases.contains_key(&alias) {
                output.aliases.insert(alias.clone(), version.clone());
            }
        }

        output.versions.push(version);
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn resolve_version(
    Json(input): Json<ResolveVersionInput>,
) -> FnResult<Json<ResolveVersionOutput>> {
    let mut output = ResolveVersionOutput::default();

    if input.initial == "node" {
        output.candidate = Some("latest".into());
    } else if input.initial == "lts-*" || input.initial == "lts/*" {
        output.candidate = Some("stable".into());
    } else if input.initial.starts_with("lts-") || input.initial.starts_with("lts/") {
        output.candidate = Some(input.initial[4..].to_owned());
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn create_shims(Json(input): Json<CreateShimsInput>) -> FnResult<Json<CreateShimsOutput>> {
    let mut global_shims = HashMap::new();

    global_shims.insert(
        "npx".into(),
        ShimConfig::global_with_alt_bin(if input.env.os == HostOS::Windows {
            "npx.cmd"
        } else {
            "bin/npx"
        }),
    );

    Ok(Json(CreateShimsOutput {
        global_shims,
        ..CreateShimsOutput::default()
    }))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec![
            ".nvmrc".into(),
            ".node-version".into(),
            "package.json".into(),
        ],
    }))
}

#[plugin_fn]
pub fn parse_version_file(
    Json(input): Json<ParseVersionInput>,
) -> FnResult<Json<ParseVersionOutput>> {
    let mut version = None;

    if input.file == "package.json" {
        let package_json: PackageJson = json::from_str(&input.content)?;

        if let Some(engines) = package_json.engines {
            if let Some(constraint) = engines.get(BIN) {
                version = Some(constraint.to_owned());
            }
        }
    } else {
        version = Some(input.content.trim().to_owned());
    }

    Ok(Json(ParseVersionOutput { version }))
}
