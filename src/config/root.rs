use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub modules: Modules,
}

impl Cfg {
    pub fn build_path(base: impl AsRef<Path>) -> PathBuf {
        const FILE_NAME: &'static str = "bicepx.json";
        base.as_ref().join(FILE_NAME)
    }
}

impl json::Load for Cfg {}
impl json::Save for Cfg {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modules {
    pub glob: PathBuf,
}
