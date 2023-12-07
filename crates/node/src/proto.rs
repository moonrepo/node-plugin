use extism_pdk::*;
use node_common::{
    commands::{self, get_global_prefix},
    NodeDistLTS, NodeDistVersion, PackageJson, PluginConfig,
};
use proto_pdk::*;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
    fn host_log(input: Json<HostLogInput>);
}

static NAME: &str = "Node.js";
static BIN: &str = "node";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        ..ToolMetadataOutput::default()
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
        ignore: vec!["node_modules".into()],
    }))
}

#[plugin_fn]
pub fn parse_version_file(
    Json(input): Json<ParseVersionFileInput>,
) -> FnResult<Json<ParseVersionFileOutput>> {
    let mut version = None;

    if input.file == "package.json" {
        if let Ok(package_json) = json::from_str::<PackageJson>(&input.content) {
            if let Some(engines) = package_json.engines {
                if let Some(constraint) = engines.get(BIN) {
                    version = Some(UnresolvedVersionSpec::parse(constraint)?);
                }
            }
        }
    } else {
        version = Some(UnresolvedVersionSpec::parse(input.content)?);
    }

    Ok(Json(ParseVersionFileOutput { version }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let mut output = LoadVersionsOutput::default();
    let response: Vec<NodeDistVersion> =
        fetch_url("https://nodejs.org/download/release/index.json")?;

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

    output
        .aliases
        .insert("latest".into(), output.latest.clone().unwrap());

    Ok(Json(output))
}

#[plugin_fn]
pub fn resolve_version(
    Json(input): Json<ResolveVersionInput>,
) -> FnResult<Json<ResolveVersionOutput>> {
    let mut output = ResolveVersionOutput::default();

    if let UnresolvedVersionSpec::Alias(alias) = input.initial {
        let candidate = if alias == "node" {
            "latest"
        } else if alias == "lts" || alias == "lts-latest" || alias == "lts-*" || alias == "lts/*" {
            "stable"
        } else if alias.starts_with("lts-") || alias.starts_with("lts/") {
            &alias[4..]
        } else {
            return Ok(Json(output));
        };

        output.candidate = Some(UnresolvedVersionSpec::Alias(candidate.to_owned()));
    }

    Ok(Json(output))
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
        _ => unreachable!(),
    };

    Ok(arch)
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_proto_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64, HostArch::Arm64, HostArch::Arm, HostArch::Powerpc64, HostArch::S390x],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64, HostArch::X86, HostArch::Arm64],
        ],
    )?;

    let arch = map_arch(env.os, env.arch)?;
    let mut version = input.context.version;
    let mut host = "https://nodejs.org/download/release".to_owned();

    // When canary, extract the latest version from the index
    if version.is_canary() {
        let response: Vec<NodeDistVersion> =
            fetch_url("https://nodejs.org/download/nightly/index.json")?;

        host = "https://nodejs.org/download/nightly".into();
        version = VersionSpec::parse(&response[0].version)?;
    }

    let prefix = match env.os {
        HostOS::Linux => format!("node-v{version}-linux-{arch}"),
        HostOS::MacOS => {
            let parsed_version = match &version {
                VersionSpec::Version(v) => v.to_owned(),
                _ => Version::new(20, 0, 0), // Doesn't matter
            };

            // Arm64 support was added after v16, but M1/M2 machines can
            // run x64 binaries via Rosetta. This is a compat hack!
            if env.arch == HostArch::Arm64 && parsed_version.major < 16 {
                format!("node-v{version}-darwin-x64")
            } else {
                format!("node-v{version}-darwin-{arch}")
            }
        }
        HostOS::Windows => format!("node-v{version}-win-{arch}"),
        _ => unreachable!(),
    };

    let filename = if env.os == HostOS::Windows {
        format!("{prefix}.zip")
    } else {
        format!("{prefix}.tar.xz")
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(prefix),
        download_url: format!("{host}/v{version}/{filename}"),
        download_name: Some(filename),
        checksum_url: Some(format!("{host}/v{version}/SHASUMS256.txt")),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_proto_environment()?;

    Ok(Json(LocateExecutablesOutput {
        globals_lookup_dirs: vec!["$PROTO_HOME/tools/node/globals/bin".into()],
        primary: Some(ExecutableConfig::new(if env.os == HostOS::Windows {
            format!("{}.exe", BIN)
        } else {
            format!("bin/{}", BIN)
        })),
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn install_global(
    Json(input): Json<InstallGlobalInput>,
) -> FnResult<Json<InstallGlobalOutput>> {
    let env = get_proto_environment()?;

    let result = exec_command!(commands::install_global(
        &input.dependency,
        get_global_prefix(&env, &input.globals_dir),
    ));

    Ok(Json(InstallGlobalOutput::from_exec_command(result)))
}

#[plugin_fn]
pub fn uninstall_global(
    Json(input): Json<UninstallGlobalInput>,
) -> FnResult<Json<UninstallGlobalOutput>> {
    let env = get_proto_environment()?;

    let result = exec_command!(commands::uninstall_global(
        &input.dependency,
        get_global_prefix(&env, &input.globals_dir),
    ));

    Ok(Json(UninstallGlobalOutput::from_exec_command(result)))
}

#[plugin_fn]
pub fn post_install(Json(input): Json<InstallHook>) -> FnResult<()> {
    let config = get_tool_config::<PluginConfig>()?;

    if !config.bundled_npm
        || input
            .passthrough_args
            .iter()
            .any(|arg| arg == "--no-bundled-npm")
    {
        return Ok(());
    }

    host_log!("Installing npm that comes bundled with Node.js");

    let mut args = vec!["install", "npm", "bundled"];

    if input.pinned {
        args.push("--pin");
    }

    let passthrough_args = input
        .passthrough_args
        .iter()
        .filter_map(|arg| {
            if arg.as_str() == "--no-bundled-npm" {
                None
            } else {
                Some(arg.as_str())
            }
        })
        .collect::<Vec<_>>();

    if !passthrough_args.is_empty() {
        args.push("--");
        args.extend(passthrough_args);
    }

    exec_command!(inherit, "proto", args);

    Ok(())
}

// DEPRECATED
// Remove in v0.23!

#[plugin_fn]
pub fn locate_bins(Json(_): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    let env = get_proto_environment()?;

    Ok(Json(LocateBinsOutput {
        bin_path: Some(if env.os == HostOS::Windows {
            format!("{}.exe", BIN).into()
        } else {
            format!("bin/{}", BIN).into()
        }),
        fallback_last_globals_dir: true,
        globals_lookup_dirs: vec!["$PROTO_HOME/tools/node/globals/bin".into()],
        ..LocateBinsOutput::default()
    }))
}
