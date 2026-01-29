use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct RootCfg {
    pub modules: ModulesCfg,
}

impl RootCfg {
    pub fn build_path(base: impl AsRef<Path>) -> PathBuf {
        const FILE_NAME: &'static str = "bicepx.json";
        base.as_ref().join(FILE_NAME)
    }
}

impl json::Load for RootCfg {}
impl json::Save for RootCfg {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulesCfg {
    pub glob: PathBuf,
}
