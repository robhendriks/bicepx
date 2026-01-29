use std::path::{Path, PathBuf};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::config::json;

pub const FILE_NAME: &'static str = "module.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub name: String,
    pub version: Version,
}

impl Cfg {
    pub fn build_path(base: impl AsRef<Path>) -> PathBuf {
        base.as_ref().join(FILE_NAME)
    }

    pub fn default() -> Self {
        Cfg {
            name: String::from(""),
            version: Version::new(0, 0, 0),
        }
    }
}

impl json::Load for Cfg {}
impl json::Save for Cfg {}
