use std::path::{Path, PathBuf};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::config::json;

pub const FILE_NAME: &'static str = "module.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub name: String,
    pub main: String,
    pub version: Version,
    pub tags: Vec<String>,
}

impl Cfg {
    pub fn build_path(base: impl AsRef<Path>) -> PathBuf {
        base.as_ref().join(FILE_NAME)
    }

    pub fn new(name: &str, main: &str, tags: Vec<String>) -> Self {
        Cfg {
            name: name.to_owned(),
            main: main.to_owned(),
            version: Version::new(0, 1, 0),
            tags,
        }
    }
}

impl json::Load for Cfg {}
impl json::Save for Cfg {}
