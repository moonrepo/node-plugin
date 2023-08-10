use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

mod npm {
    use super::*;

    generate_resolve_versions_tests!("npm-test", {
        "7" => "7.24.2",
        "8.1" => "8.1.4",
        "9.7.2" => "9.7.2",
    });

    #[test]
    fn doesnt_parse_package_manager_if_diff_name() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("npm-test", sandbox.path());

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "yarn@1.2.3" }"#.into(),
                file: "package.json".into(),
                env: Environment {
                    id: "npm-test".into(),
                    ..Environment::default()
                },
            }),
            ParseVersionFileOutput { version: None }
        );
    }

    #[test]
    fn parses_package_manager() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("npm-test", sandbox.path());

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "npm@1.2.3" }"#.into(),
                file: "package.json".into(),
                env: Environment {
                    id: "npm-test".into(),
                    ..Environment::default()
                },
            }),
            ParseVersionFileOutput {
                version: Some("1.2.3".into()),
            }
        );
    }

    #[test]
    fn parses_package_manager_latest() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("npm-test", sandbox.path());

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "npm" }"#.into(),
                file: "package.json".into(),
                env: Environment {
                    id: "npm-test".into(),
                    ..Environment::default()
                },
            }),
            ParseVersionFileOutput {
                version: Some("latest".into()),
            }
        );
    }
}

mod pnpm {
    use super::*;

    generate_resolve_versions_tests!("pnpm-test", {
        "7" => "7.33.6",
        "8.1" => "8.1.1",
        "dev" => "6.23.7-202112041634",
    });

    #[test]
    fn doesnt_parse_package_manager_if_diff_name() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("pnpm-test", sandbox.path());

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "yarn@1.2.3" }"#.into(),
                file: "package.json".into(),
                env: Environment {
                    id: "pnpm-test".into(),
                    ..Environment::default()
                },
            }),
            ParseVersionFileOutput { version: None }
        );
    }

    #[test]
    fn parses_package_manager() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("pnpm-test", sandbox.path());

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "pnpm@1.2.3" }"#.into(),
                file: "package.json".into(),
                env: Environment {
                    id: "pnpm-test".into(),
                    ..Environment::default()
                },
            }),
            ParseVersionFileOutput {
                version: Some("1.2.3".into()),
            }
        );
    }

    #[test]
    fn parses_package_manager_latest() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("pnpm-test", sandbox.path());

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "pnpm" }"#.into(),
                file: "package.json".into(),
                env: Environment {
                    id: "pnpm-test".into(),
                    ..Environment::default()
                },
            }),
            ParseVersionFileOutput {
                version: Some("latest".into()),
            }
        );
    }
}

mod yarn {
    use super::*;

    generate_resolve_versions_tests!("yarn-test", {
        "1" => "1.22.19",
        "2" => "2.4.3",
        "3" => "3.6.1",
        "berry" => "3.6.1",
    });

    #[test]
    fn doesnt_parse_package_manager_if_diff_name() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("yarn-test", sandbox.path());

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "pnpm@1.2.3" }"#.into(),
                file: "package.json".into(),
                env: Environment {
                    id: "yarn-test".into(),
                    ..Environment::default()
                },
            }),
            ParseVersionFileOutput { version: None }
        );
    }

    #[test]
    fn parses_package_manager() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("yarn-test", sandbox.path());

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "yarn@1.2.3" }"#.into(),
                file: "package.json".into(),
                env: Environment {
                    id: "yarn-test".into(),
                    ..Environment::default()
                },
            }),
            ParseVersionFileOutput {
                version: Some("1.2.3".into()),
            }
        );
    }

    #[test]
    fn parses_package_manager_latest() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("yarn-test", sandbox.path());

        assert_eq!(
            plugin.parse_version_file(ParseVersionFileInput {
                content: r#"{ "packageManager": "yarn" }"#.into(),
                file: "package.json".into(),
                env: Environment {
                    id: "yarn-test".into(),
                    ..Environment::default()
                },
            }),
            ParseVersionFileOutput {
                version: Some("latest".into()),
            }
        );
    }
}
