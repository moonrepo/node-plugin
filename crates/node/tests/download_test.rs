use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

generate_download_install_tests!("node-test", "18.0.0");

mod canary {
    use super::*;

    generate_download_install_tests!("node-test", "canary");
}

#[test]
fn supports_linux_arm64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-arm64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-arm64.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-arm64.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_arm() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-armv7l".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-armv7l.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-armv7l.tar.xz"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-x64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-x64.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-x64.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_s390x() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::S390x,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-s390x".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-s390x.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-s390x.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_ppc64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Powerpc64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-linux-ppc64le".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-linux-ppc64le.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-linux-ppc64le.tar.xz"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::MacOS,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-darwin-arm64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-darwin-arm64.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-darwin-arm64.tar.xz"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::MacOS,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-darwin-x64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-darwin-x64.tar.xz".into()),
            download_url:
                "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-darwin-x64.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_arm64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-win-arm64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-win-arm64.zip".into()),
            download_url: "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-win-arm64.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-win-x64".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-win-x64.zip".into()),
            download_url: "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-win-x64.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x86() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("node-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X86,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: "20.0.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("node-v20.0.0-win-x86".into()),
            checksum_url: Some("https://nodejs.org/download/release/v20.0.0/SHASUMS256.txt".into()),
            download_name: Some("node-v20.0.0-win-x86.zip".into()),
            download_url: "https://nodejs.org/download/release/v20.0.0/node-v20.0.0-win-x86.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("bun-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                context: ToolContext {
                    version: "20.0.0".into(),
                    ..Default::default()
                },
            })
            .bin_path,
        Some("bin/node".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("bun-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                context: ToolContext {
                    version: "20.0.0".into(),
                    ..Default::default()
                },
            })
            .bin_path,
        Some("node.exe".into())
    );
}
