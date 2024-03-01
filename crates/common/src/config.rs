#[derive(Debug, Default, serde::Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct PluginConfig {
    pub bundled_npm: bool,
    pub shared_globals_dir: bool,
}
