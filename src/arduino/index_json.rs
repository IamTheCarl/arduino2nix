use std::collections::HashMap;

use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexFile {
    pub packages: Vec<Package>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub platforms: HashMap<String, Platform>,
    pub tools: HashMap<String, Tool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    pub name: String,
    pub architecture: String,
    pub version: Version,
    pub category: String,
    pub help: Help,
    pub url: Url,
    pub archive_file_name: String,
    pub checksum: String,
    pub size: usize,
    pub boards: Vec<Board>,
    pub tools_dependencies: Vec<ToolDependency>,
    pub discovery_dependencies: Vec<DiscoveryMonitorDependency>,
    pub monitor_dependencies: Vec<DiscoveryMonitorDependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryMonitorDependency {
    pub packager: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolDependency {
    pub packager: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Help {
    #[serde(default)]
    pub online: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub version: String,
    pub systems: Vec<System>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub size: usize,
    pub checksum: String,
    pub host: String,
    pub archived_file_name: String,
    pub url: Url,
}
