use proto_pdk::*;
use proto_pdk_test_utils::create_plugin;
use starbase_sandbox::create_empty_sandbox;

#[test]
fn registers_metadata() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.register_tool(ToolMetadataInput::default()),
        ToolMetadataOutput {
            name: "Node.js".into(),
            env_vars: vec!["NODE_OPTIONS".into(), "NODE_PATH".into()],
            ..ToolMetadataOutput::default()
        }
    );
}