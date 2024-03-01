use proto_pdk_api::RunHook;
use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;
use std::collections::HashMap;
use std::path::PathBuf;

mod pre_run {
    use super::*;

    fn create_globals_dir() -> VirtualPath {
        VirtualPath::WithReal {
            path: PathBuf::from("/proto/tools/node/globals/bin"),
            virtual_prefix: PathBuf::from("/proto"),
            real_prefix: PathBuf::from("/.proto"),
        }
    }

    mod npm {
        use super::*;

        #[test]
        fn does_nothing_if_not_configured() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("npm-test", sandbox.path());

            let result = plugin.pre_run(RunHook::default());

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn does_nothing_if_disabled() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "npm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":false}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook::default());

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn does_nothing_if_enabled_but_no_args() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "npm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                ..RunHook::default()
            });

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn does_nothing_if_a_prefix_was_provided() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "npm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                passthrough_args: vec![
                    "install".into(),
                    "-g".into(),
                    "typescript".into(),
                    "--prefix".into(),
                    "/some/thing".into(),
                ],
                ..RunHook::default()
            });

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn adds_env_var() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "npm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                passthrough_args: vec!["install".into(), "-g".into(), "typescript".into()],
                ..RunHook::default()
            });

            assert_eq!(result.args, None);
            assert_eq!(
                result.env,
                Some(HashMap::from_iter([(
                    "PREFIX".into(),
                    if cfg!(windows) {
                        "/.proto/tools/node/globals/bin".into()
                    } else {
                        "/.proto/tools/node/globals".into()
                    }
                )]))
            );
        }

        #[test]
        fn adds_env_var_with_aliases() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "npm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                passthrough_args: vec!["add".into(), "--global".into(), "typescript".into()],
                ..RunHook::default()
            });

            assert!(result.env.is_some());
        }
    }

    mod pnpm {
        use super::*;

        #[test]
        fn does_nothing_if_not_configured() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("pnpm-test", sandbox.path());

            let result = plugin.pre_run(RunHook::default());

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn does_nothing_if_disabled() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "pnpm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":false}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook::default());

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn does_nothing_if_enabled_but_no_args() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "pnpm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                ..RunHook::default()
            });

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn does_nothing_if_a_dir_was_provided() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "pnpm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                passthrough_args: vec![
                    "add".into(),
                    "-g".into(),
                    "typescript".into(),
                    "--global-dir".into(),
                    "/some/thing".into(),
                ],
                ..RunHook::default()
            });

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn adds_args() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "pnpm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                passthrough_args: vec!["add".into(), "-g".into(), "typescript".into()],
                ..RunHook::default()
            });

            assert_eq!(
                result.args.as_ref().unwrap().iter().collect::<Vec<_>>(),
                vec![
                    "--global-dir",
                    "/.proto/tools/node/globals",
                    "--global-bin-dir",
                    "/.proto/tools/node/globals/bin"
                ]
            );
            assert_eq!(result.env, None);
        }

        #[test]
        fn adds_args_with_aliases() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "pnpm-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                passthrough_args: vec!["remove".into(), "--global".into(), "typescript".into()],
                ..RunHook::default()
            });

            assert!(result.args.is_some());
        }
    }

    mod yarn {
        use super::*;

        #[test]
        fn does_nothing_if_not_configured() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("yarn-test", sandbox.path());

            let result = plugin.pre_run(RunHook::default());

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn does_nothing_if_disabled() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "yarn-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":false}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook::default());

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn does_nothing_if_enabled_but_no_args() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "yarn-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                ..RunHook::default()
            });

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn does_nothing_if_a_prefix_was_provided() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "yarn-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                passthrough_args: vec![
                    "global".into(),
                    "add".into(),
                    "typescript".into(),
                    "--prefix".into(),
                    "/some/thing".into(),
                ],
                ..RunHook::default()
            });

            assert_eq!(result.args, None);
            assert_eq!(result.env, None);
        }

        #[test]
        fn adds_env_var() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin_with_config(
                "yarn-test",
                sandbox.path(),
                HashMap::from_iter([(
                    "proto_tool_config".into(),
                    r#"{"shared-globals-dir":true}"#.to_owned(),
                )]),
            );

            let result = plugin.pre_run(RunHook {
                globals_dir: Some(create_globals_dir()),
                passthrough_args: vec!["global".into(), "add".into(), "typescript".into()],
                ..RunHook::default()
            });

            assert_eq!(result.args, None);
            assert_eq!(
                result.env,
                Some(HashMap::from_iter([(
                    "PREFIX".into(),
                    "/.proto/tools/node/globals".into()
                )]))
            );
        }
    }
}
