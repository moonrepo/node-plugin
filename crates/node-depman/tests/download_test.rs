use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;
use std::collections::HashMap;

mod npm {
    use super::*;

    generate_download_install_tests!("npm-test", "9.0.0");

    #[test]
    fn supports_prebuilt() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin_with_config(
            "npm-test",
            sandbox.path(),
            HashMap::from_iter([map_config_environment(HostOS::Linux, HostArch::Arm64)]),
        );

        assert_eq!(
            plugin.download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("9.0.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            DownloadPrebuiltOutput {
                archive_prefix: Some("package".into()),
                download_url: "https://registry.npmjs.org/npm/-/npm-9.0.0.tgz".into(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn locates_default_bin() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin_with_config(
            "npm-test",
            sandbox.path(),
            HashMap::from_iter([map_config_environment(HostOS::Linux, HostArch::Arm64)]),
        );

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("9.0.0").unwrap(),
                        ..Default::default()
                    },
                })
                .primary
                .unwrap()
                .exe_path,
            Some("bin/npm-cli.js".into())
        );
    }
}

mod pnpm {
    use super::*;

    generate_download_install_tests!("pnpm-test", "8.0.0");

    #[test]
    fn supports_prebuilt() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin_with_config(
            "pnpm-test",
            sandbox.path(),
            HashMap::from_iter([map_config_environment(HostOS::Windows, HostArch::X64)]),
        );

        assert_eq!(
            plugin.download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("8.0.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            DownloadPrebuiltOutput {
                archive_prefix: Some("package".into()),
                download_url: "https://registry.npmjs.org/pnpm/-/pnpm-8.0.0.tgz".into(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn locates_default_bin() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin_with_config(
            "pnpm-test",
            sandbox.path(),
            HashMap::from_iter([map_config_environment(HostOS::Windows, HostArch::X64)]),
        );

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("8.0.0").unwrap(),
                        ..Default::default()
                    },
                })
                .primary
                .unwrap()
                .exe_path,
            Some("bin/pnpm.cjs".into())
        );
    }
}

mod yarn {
    use super::*;

    generate_download_install_tests!("yarn-test", "1.22.0");

    #[test]
    fn supports_prebuilt() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin_with_config(
            "yarn-test",
            sandbox.path(),
            HashMap::from_iter([map_config_environment(HostOS::MacOS, HostArch::X64)]),
        );

        assert_eq!(
            plugin.download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.22.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            DownloadPrebuiltOutput {
                archive_prefix: Some("yarn-v1.22.0".into()),
                download_url: "https://registry.npmjs.org/yarn/-/yarn-1.22.0.tgz".into(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn locates_default_bin() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin_with_config(
            "yarn-test",
            sandbox.path(),
            HashMap::from_iter([map_config_environment(HostOS::MacOS, HostArch::X64)]),
        );

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("1.22.0").unwrap(),
                        ..Default::default()
                    },
                })
                .primary
                .unwrap()
                .exe_path,
            Some("bin/yarn.js".into())
        );
    }
}

mod yarn_berry {
    use super::*;

    generate_download_install_tests!("yarn-test", "3.6.1");

    #[test]
    fn supports_prebuilt() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin_with_config(
            "yarn-test",
            sandbox.path(),
            HashMap::from_iter([map_config_environment(HostOS::MacOS, HostArch::X64)]),
        );

        assert_eq!(
            plugin.download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("3.6.1").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            DownloadPrebuiltOutput {
                archive_prefix: Some("package".into()),
                download_url: "https://registry.npmjs.org/@yarnpkg/cli-dist/-/cli-dist-3.6.1.tgz"
                    .into(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn locates_default_bin() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin_with_config(
            "yarn-test",
            sandbox.path(),
            HashMap::from_iter([map_config_environment(HostOS::MacOS, HostArch::X64)]),
        );

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.6.1").unwrap(),
                        ..Default::default()
                    },
                })
                .primary
                .unwrap()
                .exe_path,
            Some("bin/yarn.js".into())
        );
    }
}
