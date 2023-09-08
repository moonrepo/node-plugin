use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

generate_resolve_versions_tests!("node-test", {
    "8" => "8.17.0",
    "10.1" => "10.1.0",
    "lts-gallium" => "16.20.2",
    "lts/fermium" => "14.21.3",
    "stable" => "18.17.1",
    "node" => "20.5.1",
});

#[test]
fn loads_versions_from_dist_url() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}

#[test]
fn sets_latest_alias() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}

#[test]
fn sets_lts_aliases() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());
    let mut aliases = output.aliases.keys().collect::<Vec<_>>();
    aliases.sort();

    assert_eq!(
        aliases,
        [
            "argon", "boron", "carbon", "dubnium", "erbium", "fermium", "gallium", "hydrogen",
            "latest", "stable"
        ]
    );
}

#[test]
fn parses_engines() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("node-test", sandbox.path());

    assert_eq!(
        plugin.parse_version_file(ParseVersionFileInput {
            content: r#"{ "engines": { "node": ">=16" } }"#.into(),
            file: "package.json".into(),
        }),
        ParseVersionFileOutput {
            version: Some(">=16".into()),
        }
    );
}
