#[derive(Debug, serde::Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct PluginConfig {
    pub bundled_npm: bool,
    pub intercept_globals: bool,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            bundled_npm: true,
            intercept_globals: true,
        }
    }
}
