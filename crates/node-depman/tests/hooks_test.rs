// Importing proto_pdk crashes Windows because it contains WASM code
#[cfg(not(windows))]
mod hooks {
    use proto_pdk::{RunHook, ToolContext};
    use proto_pdk_test_utils::create_plugin;
    use starbase_sandbox::create_empty_sandbox;
    use std::env;

    mod npm {
        use super::*;

        #[test]
        fn does_nothing_if_no_args() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("npm-test", sandbox.path());

            plugin.pre_run(RunHook::default());
        }

        #[test]
        fn skips_when_env_var_set() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("npm-test", sandbox.path());

            env::set_var("PROTO_INSTALL_GLOBAL", "1");

            plugin.pre_run(RunHook {
                passthrough_args: vec!["install".into(), "-g".into(), "typescript".into()],
                context: ToolContext::default(),
            });

            env::remove_var("PROTO_INSTALL_GLOBAL");
        }

        #[test]
        fn can_bypass_with_user_config() {
            let sandbox = create_empty_sandbox();
            let mut plugin = create_plugin("npm-test", sandbox.path());

            plugin.tool.plugin.manifest.config.insert(
                "proto_tool_config".into(),
                r#"{"intercept-globals":false}"#.to_owned(),
            );

            plugin.tool.plugin.reload_config().unwrap();

            plugin.pre_run(RunHook {
                passthrough_args: vec!["install".into(), "-g".into(), "typescript".into()],
                ..RunHook::default()
            });
        }

        #[test]
        #[should_panic(expected = "Global binaries must be installed")]
        fn errors_if_installing_global() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("npm-test", sandbox.path());

            plugin.pre_run(RunHook {
                passthrough_args: vec!["install".into(), "-g".into(), "typescript".into()],
                ..RunHook::default()
            });
        }

        #[test]
        fn doesnt_error_for_other_commands() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("npm-test", sandbox.path());

            plugin.pre_run(RunHook {
                passthrough_args: vec!["info".into(), "--json".into(), "typescript".into()],
                ..RunHook::default()
            });
        }
    }

    mod pnpm {
        use super::*;

        #[test]
        #[should_panic(expected = "Global binaries must be installed")]
        fn errors_if_installing_global() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("pnpm-test", sandbox.path());

            plugin.pre_run(RunHook {
                passthrough_args: vec!["add".into(), "--global".into(), "typescript".into()],
                ..RunHook::default()
            });
        }

        #[test]
        fn doesnt_error_for_other_commands() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("pnpm-test", sandbox.path());

            plugin.pre_run(RunHook {
                passthrough_args: vec!["info".into(), "--json".into(), "typescript".into()],
                ..RunHook::default()
            });
        }
    }

    mod yarn {
        use super::*;

        #[test]
        #[should_panic(expected = "Global binaries must be installed")]
        fn errors_if_installing_global() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("yarn-test", sandbox.path());

            plugin.pre_run(RunHook {
                passthrough_args: vec!["global".into(), "add".into(), "typescript".into()],
                ..RunHook::default()
            });
        }

        #[test]
        fn doesnt_error_for_other_commands() {
            let sandbox = create_empty_sandbox();
            let plugin = create_plugin("yarn-test", sandbox.path());

            plugin.pre_run(RunHook {
                passthrough_args: vec!["info".into(), "--json".into(), "typescript".into()],
                ..RunHook::default()
            });
        }
    }
}
