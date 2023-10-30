use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

fn create_metadata(id: &str) -> ToolMetadataInput {
    ToolMetadataInput { id: id.into() }
}

mod npm {
    use super::*;

    #[test]
    fn registers_metadata() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("npm-test", sandbox.path());

        assert_eq!(
            plugin.register_tool(create_metadata("npm-test")),
            ToolMetadataOutput {
                name: "npm".into(),
                type_of: PluginType::DependencyManager,
                default_version: Some(UnresolvedVersionSpec::Alias("bundled".into())),
                plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
                ..ToolMetadataOutput::default()
            }
        );
    }
}

mod pnpm {
    use super::*;

    #[test]
    fn registers_metadata() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("pnpm-test", sandbox.path());

        assert_eq!(
            plugin.register_tool(create_metadata("pnpm-test")),
            ToolMetadataOutput {
                name: "pnpm".into(),
                type_of: PluginType::DependencyManager,
                plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
                ..ToolMetadataOutput::default()
            }
        );
    }
}

mod yarn {
    use super::*;

    #[test]
    fn registers_metadata() {
        let sandbox = create_empty_sandbox();
        let plugin = create_plugin("yarn-test", sandbox.path());

        assert_eq!(
            plugin.register_tool(create_metadata("yarn-test")),
            ToolMetadataOutput {
                name: "yarn".into(),
                type_of: PluginType::DependencyManager,
                plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
                ..ToolMetadataOutput::default()
            }
        );
    }
}
