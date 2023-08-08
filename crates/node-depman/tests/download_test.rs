use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;
use std::path::PathBuf;

mod npm {
    use super::*;

    generate_download_install_tests!("npm-test", "9.0.0");

    #[test]
    fn supports_prebuilt() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("npm-test", sandbox.path());

        assert_eq!(
            plugin.download_prebuilt(DownloadPrebuiltInput {
                env: Environment {
                    arch: HostArch::Arm64,
                    id: "npm".into(),
                    os: HostOS::Linux,
                    version: "9.0.0".into(),
                    ..Default::default()
                }
            }),
            DownloadPrebuiltOutput {
                archive_prefix: Some("package".into()),
                checksum_name: None,
                checksum_url: None,
                download_name: None,
                download_url: "https://registry.npmjs.org/npm/-/npm-9.0.0.tgz".into()
            }
        );
    }

    #[test]
    fn locates_default_bin() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("npm-test", sandbox.path());

        assert_eq!(
            plugin
                .locate_bins(LocateBinsInput {
                    env: Environment {
                        arch: HostArch::Arm64,
                        id: "npm".into(),
                        os: HostOS::Linux,
                        version: "9.0.0".into(),
                        ..Default::default()
                    },
                    home_dir: PathBuf::new(),
                    tool_dir: PathBuf::new(),
                })
                .bin_path,
            Some("bin/npm".into())
        );
    }
}

mod pnpm {
    use super::*;

    generate_download_install_tests!("pnpm-test", "8.0.0");

    #[test]
    fn supports_prebuilt() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("pnpm-test", sandbox.path());

        assert_eq!(
            plugin.download_prebuilt(DownloadPrebuiltInput {
                env: Environment {
                    arch: HostArch::X64,
                    id: "pnpm".into(),
                    os: HostOS::Windows,
                    version: "8.0.0".into(),
                    ..Default::default()
                }
            }),
            DownloadPrebuiltOutput {
                archive_prefix: Some("package".into()),
                checksum_name: None,
                checksum_url: None,
                download_name: None,
                download_url: "https://registry.npmjs.org/pnpm/-/pnpm-8.0.0.tgz".into()
            }
        );
    }

    #[test]
    fn locates_default_bin() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("pnpm-test", sandbox.path());

        assert_eq!(
            plugin
                .locate_bins(LocateBinsInput {
                    env: Environment {
                        arch: HostArch::X64,
                        id: "pnpm".into(),
                        os: HostOS::Windows,
                        version: "8.0.0".into(),
                        ..Default::default()
                    },
                    home_dir: PathBuf::new(),
                    tool_dir: PathBuf::new(),
                })
                .bin_path,
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
        let plugin = create_plugin("yarn-test", sandbox.path());

        assert_eq!(
            plugin.download_prebuilt(DownloadPrebuiltInput {
                env: Environment {
                    arch: HostArch::X64,
                    id: "yarn".into(),
                    os: HostOS::MacOS,
                    version: "1.22.0".into(),
                    ..Default::default()
                }
            }),
            DownloadPrebuiltOutput {
                archive_prefix: Some("yarn-v1.22.0".into()),
                checksum_name: None,
                checksum_url: None,
                download_name: None,
                download_url: "https://registry.npmjs.org/yarn/-/yarn-1.22.0.tgz".into()
            }
        );
    }

    #[test]
    fn locates_default_bin() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("yarn-test", sandbox.path());

        assert_eq!(
            plugin
                .locate_bins(LocateBinsInput {
                    env: Environment {
                        arch: HostArch::X64,
                        id: "yarn".into(),
                        os: HostOS::MacOS,
                        version: "1.22.0".into(),
                        ..Default::default()
                    },
                    home_dir: PathBuf::new(),
                    tool_dir: PathBuf::new(),
                })
                .bin_path,
            Some("bin/yarn".into())
        );
    }
}

mod yarn_berry {
    use super::*;

    generate_download_install_tests!("yarn-test", "3.6.1");

    #[test]
    fn supports_prebuilt() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("yarn-test", sandbox.path());

        assert_eq!(
            plugin.download_prebuilt(DownloadPrebuiltInput {
                env: Environment {
                    arch: HostArch::X64,
                    id: "yarn".into(),
                    os: HostOS::MacOS,
                    version: "3.6.1".into(),
                    ..Default::default()
                }
            }),
            DownloadPrebuiltOutput {
                archive_prefix: Some("package".into()),
                checksum_name: None,
                checksum_url: None,
                download_name: None,
                download_url: "https://registry.npmjs.org/@yarnpkg/cli-dist/-/cli-dist-3.6.1.tgz"
                    .into()
            }
        );
    }

    #[test]
    fn locates_default_bin() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("yarn-test", sandbox.path());

        assert_eq!(
            plugin
                .locate_bins(LocateBinsInput {
                    env: Environment {
                        arch: HostArch::X64,
                        id: "yarn".into(),
                        os: HostOS::MacOS,
                        version: "3.6.1".into(),
                        ..Default::default()
                    },
                    home_dir: PathBuf::new(),
                    tool_dir: PathBuf::new(),
                })
                .bin_path,
            Some("bin/yarn".into())
        );
    }
}

#[test]
fn locates_bin_from_package_json_bin() {
    let sandbox = create_empty_sandbox();

    sandbox.create_file(
        ".proto/tools/npm-test/latest/package.json",
        r#"{
    "main": "./index.js",
    "bin": "./file.js"
}"#,
    );

    let plugin = create_plugin("npm-test", sandbox.path());

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                env: Environment {
                    arch: HostArch::X64,
                    id: "npm".into(),
                    os: HostOS::Windows,
                    version: "20.0.0".into(),
                    ..Default::default()
                },
                home_dir: PathBuf::new(),
                tool_dir: plugin.to_virtual_path(&plugin.tool.get_tool_dir()),
            })
            .bin_path,
        Some("./file.js".into())
    );

    sandbox.create_file(
        ".proto/tools/npm-test/latest/package.json",
        r#"{
    "main": "./index.js",
    "bin": {
        "npm": "./npm.js",
        "pnpm": "./pnpm.js",
        "yarn": "./yarn.js"
    }
}"#,
    );

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                env: Environment {
                    arch: HostArch::Arm64,
                    id: "npm".into(),
                    os: HostOS::Linux,
                    version: "9.0.0".into(),
                    ..Default::default()
                },
                home_dir: PathBuf::new(),
                tool_dir: plugin.to_virtual_path(&plugin.tool.get_tool_dir()),
            })
            .bin_path,
        Some("./npm.js".into())
    );
}

#[test]
fn locates_bin_from_package_json_main() {
    let sandbox = create_empty_sandbox();

    sandbox.create_file(
        ".proto/tools/npm-test/latest/package.json",
        r#"{
    "main": "./index.js"
}"#,
    );

    let plugin = create_plugin("npm-test", sandbox.path());

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                env: Environment {
                    arch: HostArch::X64,
                    id: "npm".into(),
                    os: HostOS::MacOS,
                    version: "8.0.0".into(),
                    ..Default::default()
                },
                home_dir: PathBuf::new(),
                tool_dir: plugin.to_virtual_path(&plugin.tool.get_tool_dir()),
            })
            .bin_path,
        Some("./index.js".into())
    );
}
