use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BinField {
    String(String),
    Object(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub bin: Option<BinField>,

    pub engines: Option<HashMap<String, String>>,

    pub main: Option<String>,

    pub name: Option<String>,

    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,

    pub version: Option<String>,
}
