use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    cache: CacheCfg,
}

impl Cfg {
    pub fn build_path(base: impl AsRef<Path>) -> PathBuf {
        const FILE_NAME: &'static str = "bicepx.json";
        base.as_ref().join(FILE_NAME)
    }

    pub fn new() -> Self {
        Cfg {
            cache: CacheCfg {
                dir: PathBuf::from(".bicepx"),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CacheCfg {
    dir: PathBuf,
}

impl json::Load for Cfg {}
impl json::Save for Cfg {}
