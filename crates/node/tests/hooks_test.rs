use proto_pdk::InstallHook;
use proto_pdk_test_utils::{core::AliasOrVersion, create_plugin, ToolManifest, Version};
use serial_test::serial;
use starbase_sandbox::create_empty_sandbox;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

fn set_vars(path: PathBuf) {
    env::set_var("PROTO_ROOT", path.to_string_lossy().to_string());
    env::set_var("PROTO_NODE_VERSION", "18.0.0");
}

fn reset_vars() {
    env::remove_var("PROTO_ROOT");
    env::remove_var("PROTO_NODE_VERSION");
}

mod node_hooks {
    use super::*;

    #[test]
    #[serial]
    fn installs_bundled_npm() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("node-test", sandbox.path());

        assert!(!sandbox.path().join(".proto/tools/npm/8.6.0").exists());

        set_vars(sandbox.path().join(".proto"));

        plugin.post_install(InstallHook::default());

        reset_vars();

        assert!(sandbox.path().join(".proto/tools/npm/8.6.0").exists());

        let manifest =
            ToolManifest::load(sandbox.path().join(".proto/tools/npm/manifest.json")).unwrap();

        assert_eq!(
            manifest.default_version,
            Some(AliasOrVersion::parse("bundled").unwrap())
        );
        assert_eq!(
            manifest.installed_versions,
            HashSet::from_iter([Version::parse("8.6.0").unwrap()])
        );
    }

    #[test]
    #[serial]
    fn can_pin_bundled_npm() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("node-test", sandbox.path());

        set_vars(sandbox.path().join(".proto"));

        plugin.post_install(InstallHook {
            pinned: true,
            ..InstallHook::default()
        });

        reset_vars();

        let manifest =
            ToolManifest::load(sandbox.path().join(".proto/tools/npm/manifest.json")).unwrap();

        assert_eq!(
            manifest.default_version,
            Some(AliasOrVersion::parse("8.6.0").unwrap())
        );
    }

    #[test]
    #[serial]
    fn can_skip_bundled_npm() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("node-test", sandbox.path());

        assert!(!sandbox.path().join(".proto/tools/npm/8.6.0").exists());

        set_vars(sandbox.path().join(".proto"));

        plugin.post_install(InstallHook {
            passthrough_args: vec!["--no-bundled-npm".into()],
            ..InstallHook::default()
        });

        reset_vars();

        assert!(!sandbox.path().join(".proto/tools/npm/8.6.0").exists());
    }
}
