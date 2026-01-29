use std::path::{Path, PathBuf};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::config::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleCfg {
    pub name: String,
    pub version: Version,
}

impl ModuleCfg {
    pub fn build_path(base: impl AsRef<Path>) -> PathBuf {
        const FILE_NAME: &'static str = "module.json";
        base.as_ref().join(FILE_NAME)
    }
}

impl json::Load for ModuleCfg {}
impl json::Save for ModuleCfg {}
