use proto_pdk::*;
use proto_pdk_test_utils::{create_plugin, generate_download_install_tests};
use starbase_sandbox::create_empty_sandbox;
use std::path::PathBuf;

generate_download_install_tests!("node-test", "18.0.0");

#[test]
fn supports_linux_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Arm64,
                os: HostOS::Linux,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-arm64".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-arm64.tar.xz".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-linux-arm64.tar.xz".into()
        }
    );
}

#[test]
fn supports_linux_arm() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Arm,
                os: HostOS::Linux,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-armv7l".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-armv7l.tar.xz".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-linux-armv7l.tar.xz".into()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::X64,
                os: HostOS::Linux,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-x64".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-x64.tar.xz".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-linux-x64.tar.xz".into()
        }
    );
}

#[test]
fn supports_linux_s390x() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::S390x,
                os: HostOS::Linux,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-s390x".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-s390x.tar.xz".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-linux-s390x.tar.xz".into()
        }
    );
}

#[test]
fn supports_linux_ppc64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Powerpc64,
                os: HostOS::Linux,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-ppc64le".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-ppc64le.tar.xz".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-linux-ppc64le.tar.xz"
                .into()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Arm64,
                os: HostOS::MacOS,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-darwin-arm64".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-darwin-arm64.tar.xz".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-darwin-arm64.tar.xz".into()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::X64,
                os: HostOS::MacOS,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-darwin-x64".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-darwin-x64.tar.xz".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-darwin-x64.tar.xz".into()
        }
    );
}

#[test]
fn supports_windows_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Arm64,
                os: HostOS::Windows,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-win-arm64".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-win-arm64.zip".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-win-arm64.zip".into()
        }
    );
}

#[test]
fn supports_windows_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::X64,
                os: HostOS::Windows,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-win-x64".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-win-x64.zip".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-win-x64.zip".into()
        }
    );
}

#[test]
fn supports_windows_x86() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::X86,
                os: HostOS::Windows,
                version: "20.0.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-win-x86".into()),
            bin_path: None,
            checksum_name: None,
            checksum_url: Some("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-win-x86.zip".into()),
            download_url: "https://nodejs.org/dist/v20.0.0/node-v20.0.0-win-x86.zip".into()
        }
    );
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                env: Environment {
                    arch: HostArch::Arm64,
                    os: HostOS::Linux,
                    version: "20.0.0".into(),
                    ..Default::default()
                },
                tool_dir: PathBuf::new()
            })
            .bin_path,
        Some("bin/node".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                env: Environment {
                    arch: HostArch::X64,
                    os: HostOS::Windows,
                    version: "20.0.0".into(),
                    ..Default::default()
                },
                tool_dir: PathBuf::new()
            })
            .bin_path,
        Some("node.exe".into())
    );
}
