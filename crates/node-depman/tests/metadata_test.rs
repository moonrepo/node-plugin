// use proto_pdk::*;
// use proto_pdk_test_utils::create_plugin;
// use starbase_sandbox::create_empty_sandbox;

// mod npm {
//     use super::*;

//     #[test]
//     fn registers_metadata() {
//         let sandbox = create_empty_sandbox();
//         let plugin = create_plugin("npm-test", sandbox.path());

//         assert_eq!(
//             plugin.register_tool(ToolMetadataInput::default()),
//             ToolMetadataOutput {
//                 name: "npm".into(),
//                 type_of: PluginType::DependencyManager,
//                 ..ToolMetadataOutput::default()
//             }
//         );
//     }
// }
