use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct PackageJson {
    pub engines: Option<HashMap<String, String>>,
    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,
    pub version: Option<String>,
}
