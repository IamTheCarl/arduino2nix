use std::collections::HashMap;

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexFile {
    pub packages: Vec<Package>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub platforms: Vec<Platform>,
    pub tools: Vec<Tool>,

    /// When re-serializing, we want to restore all values we didn't know what to do with.
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    pub name: String,
    pub version: Version,
    pub url: String,
    pub archive_file_name: String,
    pub checksum: String,

    /// When re-serializing, we want to restore all values we didn't know what to do with.
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub version: String,
    pub systems: Vec<System>,

    /// When re-serializing, we want to restore all values we didn't know what to do with.
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub checksum: String,
    pub archive_file_name: String,
    pub url: String,

    /// When re-serializing, we want to restore all values we didn't know what to do with.
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
