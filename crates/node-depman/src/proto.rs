use crate::npm_registry::parse_registry_response;
use crate::package_manager::PackageManager;
use extism_pdk::*;
use node_common::{NodeDepmanPluginConfig, NodeDistVersion, VoltaField};
use nodejs_package_json::PackageJson;
use proto_pdk::*;
use std::collections::HashMap;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
    fn get_env_var(key: &str) -> String;
}

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    let manager = PackageManager::detect()?;

    Ok(Json(ToolMetadataOutput {
        name: manager.to_string(),
        type_of: PluginType::DependencyManager,
        default_version: if manager == PackageManager::Npm {
            Some(UnresolvedVersionSpec::Alias("bundled".into()))
        } else {
            None
        },
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec!["package.json".into()],
        ignore: vec!["node_modules".into()],
    }))
}

#[plugin_fn]
pub fn parse_version_file(
    Json(input): Json<ParseVersionFileInput>,
) -> FnResult<Json<ParseVersionFileOutput>> {
    let mut version = None;

    if input.file == "package.json" {
        if let Ok(mut package_json) = json::from_str::<PackageJson>(&input.content) {
            let manager_name = PackageManager::detect()?.to_string();

            if let Some(pm) = package_json.package_manager {
                let mut parts = pm.split('@');
                let name = parts.next().unwrap_or_default();

                if name == manager_name {
                    let value = if let Some(value) = parts.next() {
                        // Remove corepack build metadata hash
                        if let Some(index) = value.find('+') {
                            &value[0..index]
                        } else {
                            value
                        }
                    } else {
                        "latest"
                    };

                    version = Some(UnresolvedVersionSpec::parse(value)?);
                }
            }

            if version.is_none() {
                if let Some(engines) = package_json.engines {
                    if let Some(constraint) = engines.get(&manager_name) {
                        version = Some(UnresolvedVersionSpec::parse(constraint)?);
                    }
                }
            }

            if version.is_none() {
                if let Some(volta_raw) = package_json.other_fields.remove("volta") {
                    let volta: VoltaField = json::from_value(volta_raw)?;

                    if let Some(volta_tool_version) = match manager_name.as_str() {
                        "npm" => volta.npm,
                        "pnpm" => volta.pnpm,
                        "yarn" => volta.yarn,
                        _ => None,
                    } {
                        version = Some(UnresolvedVersionSpec::parse(volta_tool_version)?);
                    }
                }
            }
        }
    }

    Ok(Json(ParseVersionFileOutput { version }))
}

#[plugin_fn]
pub fn load_versions(Json(input): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let mut output = LoadVersionsOutput::default();
    let manager = PackageManager::detect()?;
    let package_name = manager.get_package_name(&input.initial);

    let mut map_output = |res: HttpResponse, is_yarn: bool| -> Result<(), Error> {
        let res = parse_registry_response(res, is_yarn)?;

        for item in res.versions.values() {
            output.versions.push(Version::parse(&item.version)?);
        }

        // Dist tags always includes latest
        for (alias, version) in res.dist_tags {
            let version = Version::parse(&version)?;

            if alias == "latest" {
                output.latest = Some(version.clone());

                // The berry alias only exists in the `yarn` package,
                // but not `@yarnpkg/cli-dist`, so update it here
                if is_yarn && res.name == "@yarnpkg/cli-dist" {
                    output.aliases.insert("berry".into(), version.clone());
                }
            }

            output.aliases.entry(alias).or_insert(version);
        }

        Ok(())
    };

    // Yarn is managed by 2 different packages, so we need to request versions from both of them!
    if manager == PackageManager::Yarn {
        map_output(
            fetch(HttpRequest::new("https://registry.npmjs.org/yarn/"), None)?,
            true,
        )?;
        map_output(
            fetch(
                HttpRequest::new("https://registry.npmjs.org/@yarnpkg/cli-dist/"),
                None,
            )?,
            true,
        )?;
    } else {
        map_output(
            fetch(
                HttpRequest::new(format!("https://registry.npmjs.org/{}/", package_name)),
                None,
            )?,
            false,
        )?;
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
    let manager = PackageManager::detect()?;
    let mut output = ResolveVersionOutput::default();

    match manager {
        PackageManager::Npm => {
            // When the alias "bundled" is provided, we should install the npm
            // version that comes bundled with the current Node.js version.
            if input.initial.is_alias("bundled") {
                debug!("Received the bundled alias, attempting to find a version");

                let response: Vec<NodeDistVersion> =
                    fetch_url("https://nodejs.org/download/release/index.json")?;
                let mut found_version = false;

                // Infer from proto's environment variable
                if let Some(node_version) = host_env!("PROTO_NODE_VERSION") {
                    for node_release in &response {
                        // Theirs starts with v, ours does not
                        if node_release.version[1..] == node_version && node_release.npm.is_some() {
                            output.version =
                                Some(VersionSpec::parse(node_release.npm.as_ref().unwrap())?);
                            found_version = true;
                            break;
                        }
                    }
                }

                // Otherwise call the current `node` binary and infer from that
                if !found_version {
                    let result = exec_command!("node", ["--version"]);
                    let node_version = result.stdout.trim();

                    for node_release in &response {
                        // Both start with v
                        if node_release.version == node_version && node_release.npm.is_some() {
                            output.version =
                                Some(VersionSpec::parse(node_release.npm.as_ref().unwrap())?);
                            found_version = true;
                            break;
                        }
                    }
                }

                if !found_version {
                    debug!(
                        "Could not find a bundled npm version for Node.js, falling back to latest"
                    );

                    output.candidate = Some(UnresolvedVersionSpec::Alias("latest".into()));
                }
            }
        }

        PackageManager::Yarn => {
            if let UnresolvedVersionSpec::Alias(alias) = input.initial {
                if alias == "berry" || alias == "latest" {
                    output.candidate = Some(UnresolvedVersionSpec::parse("~4")?);
                } else if alias == "legacy" || alias == "classic" {
                    output.candidate = Some(UnresolvedVersionSpec::parse("~1")?);
                }
            }
        }

        _ => {}
    };

    Ok(Json(output))
}

fn get_archive_prefix(manager: &PackageManager, spec: &VersionSpec) -> String {
    if manager.is_yarn_classic(spec.to_unresolved_spec()) {
        if let VersionSpec::Version(version) = spec {
            // Prefix changed to "package" in v1.22.20
            // https://github.com/yarnpkg/yarn/releases/tag/v1.22.20
            if version.minor <= 22 && version.patch <= 19 {
                return format!("yarn-v{version}");
            }
        }
    }

    "package".into()
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let version = &input.context.version;
    let manager = PackageManager::detect()?;

    if version.is_canary() {
        return Err(plugin_err!(PluginError::UnsupportedCanary {
            tool: manager.to_string()
        }));
    }

    let package_name = manager.get_package_name(version.to_unresolved_spec());

    let package_without_scope = if let Some(index) = package_name.find('/') {
        &package_name[index + 1..]
    } else {
        &package_name
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(get_archive_prefix(&manager, version)),
        download_url: format!(
            "https://registry.npmjs.org/{package_name}/-/{package_without_scope}-{version}.tgz",
        ),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;
    let manager = PackageManager::detect()?;
    let mut secondary = HashMap::default();
    let mut primary;

    // These are the directories that contain the executable binaries,
    // NOT where the packages/node modules are stored. Some package managers
    // have separate folders for the 2 processes, and then create symlinks.
    let mut globals_lookup_dirs = vec!["$PREFIX/bin".into()];

    // We don't link binaries for package managers for the following reasons:
    // 1 - We can't link JS files because they aren't executable.
    // 2 - We can't link the bash/cmd wrappers, as they expect the files to exist
    //     relative from the node install directory, which they do not.
    match &manager {
        PackageManager::Npm => {
            primary = ExecutableConfig::with_parent("bin/npm-cli.js", "node");
            primary.no_bin = true;

            // npx
            let mut npx = ExecutableConfig::with_parent("bin/npx-cli.js", "node");
            npx.no_bin = true;

            secondary.insert("npx".into(), npx);

            // node-gyp
            let mut node_gyp =
                ExecutableConfig::with_parent("node_modules/node-gyp/bin/node-gyp.js", "node");
            node_gyp.no_bin = true;

            secondary.insert("node-gyp".into(), node_gyp);

            // https://docs.npmjs.com/cli/v9/configuring-npm/folders#prefix-configuration
            // https://github.com/npm/cli/blob/latest/lib/npm.js
            // https://github.com/npm/cli/blob/latest/workspaces/config/lib/index.js#L339
            globals_lookup_dirs.push("$TOOL_DIR/bin".into());
        }
        PackageManager::Pnpm => {
            primary = ExecutableConfig::with_parent("bin/pnpm.cjs", "node");
            primary.no_bin = true;

            // pnpx
            secondary.insert(
                "pnpx".into(),
                ExecutableConfig {
                    no_bin: true,
                    shim_before_args: Some(StringOrVec::String("dlx".into())),
                    ..ExecutableConfig::default()
                },
            );

            // https://pnpm.io/npmrc#global-dir
            // https://github.com/pnpm/pnpm/blob/main/config/config/src/index.ts#L350
            // https://github.com/pnpm/pnpm/blob/main/config/config/src/dirs.ts#L40
            globals_lookup_dirs.push("$PNPM_HOME".into());

            if env.os == HostOS::Windows {
                globals_lookup_dirs.push("$LOCALAPPDATA\\pnpm".into());
            } else if env.os == HostOS::MacOS {
                globals_lookup_dirs.push("$HOME/Library/pnpm".into());
            } else {
                globals_lookup_dirs.push("$HOME/.local/share/pnpm".into());
            }
        }
        PackageManager::Yarn => {
            primary = ExecutableConfig::with_parent("bin/yarn.js", "node");
            primary.no_bin = true;

            // yarnpkg
            secondary.insert("yarnpkg".into(), primary.clone());

            // https://github.com/yarnpkg/yarn/blob/master/src/cli/commands/global.js#L84
            if env.os == HostOS::Windows {
                globals_lookup_dirs.push("$LOCALAPPDATA\\Yarn\\bin".into());
                globals_lookup_dirs.push("$HOME\\.yarn\\bin".into());
            } else {
                globals_lookup_dirs.push("$HOME/.yarn/bin".into());
            }
        }
    };

    let config = get_tool_config::<NodeDepmanPluginConfig>()?;

    if config.shared_globals_dir {
        globals_lookup_dirs.clear();
        globals_lookup_dirs.push("$PROTO_HOME/tools/node/globals/bin".into());
    }

    Ok(Json(LocateExecutablesOutput {
        globals_lookup_dirs,
        primary: Some(primary),
        secondary,
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn pre_run(Json(input): Json<RunHook>) -> FnResult<Json<RunHookResult>> {
    let mut result = RunHookResult::default();

    let Some(globals_dir) = &input.globals_dir else {
        return Ok(Json(result));
    };

    let args = &input.passthrough_args;
    let config = get_tool_config::<NodeDepmanPluginConfig>()?;

    if args.len() < 3 || !config.shared_globals_dir {
        return Ok(Json(result));
    }

    let env = get_host_environment()?;
    let manager = PackageManager::detect()?;

    // Includes trailing /bin folder
    let globals_bin_dir = globals_dir
        .real_path()
        .unwrap()
        .to_string_lossy()
        .to_string();
    // Parent directory, doesn't include /bin folder
    let globals_root_dir = globals_dir
        .real_path()
        .unwrap()
        .parent()
        .unwrap()
        .to_string_lossy()
        .to_string();

    match manager {
        // npm install|add|etc -g <dep>
        PackageManager::Npm => {
            let aliases = vec![
                // install
                "add",
                "i",
                "in",
                "ins",
                "inst",
                "insta",
                "instal",
                "install",
                "isnt",
                "isnta",
                "isntal",
                "isntall",
                // uninstall
                "r",
                "remove",
                "rm",
                "un",
                "uninstall",
                "unlink",
            ];

            if aliases.iter().any(|alias| *alias == args[0])
                && args
                    .iter()
                    .any(|arg| arg == "--global" || arg == "-g" || arg == "--location=global")
                && args.iter().all(|arg| arg != "--prefix")
            {
                result
                    .env
                    .get_or_insert(HashMap::default())
                    // Unix will create a /bin directory when installing into the root,
                    // while Windows installs directly into the /bin directory.
                    .insert(
                        "PREFIX".into(),
                        if env.os == HostOS::Windows {
                            globals_bin_dir
                        } else {
                            globals_root_dir
                        },
                    );
            }
        }

        // pnpm add|update|etc -g <dep>
        PackageManager::Pnpm => {
            let aliases = [
                "add", "update", "remove", "list", "outdated", "why", "root", "bin",
            ];

            if aliases.iter().any(|alias| *alias == args[0])
                && args.iter().any(|arg| arg == "--global" || arg == "-g")
                && args
                    .iter()
                    .all(|arg| arg != "--global-dir" && arg != "--global-bin-dir")
            {
                // These arguments aren't ideal, but pnpm doesn't support
                // environment variables from what I've seen...
                let new_args = result.args.get_or_insert(vec![]);
                new_args.push("--global-dir".into());
                new_args.push(globals_root_dir);
                new_args.push("--global-bin-dir".into());
                new_args.push(globals_bin_dir);
            }
        }

        // yarn global add|remove|etc <dep>
        PackageManager::Yarn => {
            let aliases = ["add", "bin", "list", "remove", "upgrade"];

            if args[0] == "global"
                && aliases.iter().any(|alias| *alias == args[1])
                && args.iter().all(|arg| arg != "--prefix")
            {
                result
                    .env
                    .get_or_insert(HashMap::default())
                    // Both Unix and Windows will create a /bin directory,
                    // when installing into the root.
                    .insert("PREFIX".into(), globals_root_dir);
            }
        }
    };

    Ok(Json(result))
}
