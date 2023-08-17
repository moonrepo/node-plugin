// TODO: Enable once 0.15 lands

// #[test]
// fn installs_bundled_npm() {
//     let temp = create_empty_sandbox();

//     let mut cmd = create_proto_command(temp.path());
//     let assert = cmd.arg("install").arg("node").arg("19.0.0").assert();

//     let output = output_to_string(&assert.get_output().stderr.to_vec());

//     assert!(predicate::str::contains("Node.js has been installed").eval(&output));
//     assert!(predicate::str::contains("npm has been installed").eval(&output));

//     assert!(temp.path().join("tools/node/19.0.0").exists());
//     assert!(temp.path().join("tools/npm/8.19.2").exists());

//     let manifest = ToolManifest::load(temp.path().join("tools/npm/manifest.json")).unwrap();

//     assert_eq!(
//         manifest.default_version,
//         Some(AliasOrVersion::parse("bundled").unwrap())
//     );
//     assert_eq!(
//         manifest.installed_versions,
//         HashSet::from_iter([Version::parse("8.19.2").unwrap()])
//     );
// }

// #[test]
// fn skips_bundled_npm() {
//     let temp = create_empty_sandbox();

//     let mut cmd = create_proto_command(temp.path());
//     let assert = cmd
//         .arg("install")
//         .arg("node")
//         .arg("19.0.0")
//         .arg("--")
//         .arg("--no-bundled-npm")
//         .assert();

//     let output = output_to_string(&assert.get_output().stderr.to_vec());

//     assert!(predicate::str::contains("Node.js has been installed").eval(&output));
//     assert!(!predicate::str::contains("npm has been installed").eval(&output));

//     assert!(temp.path().join("tools/node/19.0.0").exists());
//     assert!(!temp.path().join("tools/npm/8.19.2").exists());
// }
