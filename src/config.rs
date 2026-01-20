use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Root {
    pub modules: Modules,
}

#[derive(Deserialize, Debug)]
pub struct Modules {
    pub entrypoint: PathBuf,
}

impl Root {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Root> {
        let path = path.as_ref();

        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        toml::from_str::<Root>(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))
    }
}
