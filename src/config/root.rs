use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    build_dir: PathBuf,
}

impl Cfg {
    pub fn build_path(base: impl AsRef<Path>) -> PathBuf {
        const FILE_NAME: &'static str = "bicepx.json";
        base.as_ref().join(FILE_NAME)
    }

    pub fn new() -> Self {
        Cfg {
            build_dir: PathBuf::from(".bicepx"),
        }
    }
}

impl json::Load for Cfg {}
impl json::Save for Cfg {}
