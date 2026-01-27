use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};
use semver::Version;
use serde::{Deserialize, Serialize};

pub trait SaveAsJson: Serialize {
    async fn save_as_json(&self, path: impl AsRef<Path>, overwrite: bool) -> anyhow::Result<()> {
        let path_ref = path.as_ref();

        if !overwrite && path_ref.exists() {
            return Err(anyhow!("File already exists"));
        }

        let contents =
            serde_json::to_vec_pretty(&self).with_context(|| "Failed to serialize JSON")?;

        tokio::fs::write(&path, &contents)
            .await
            .with_context(|| "Failed to write JSON to file")?;

        Ok(())
    }
}

pub trait LoadFromJson: for<'de> Deserialize<'de> {
    async fn load_from_json(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let contents = tokio::fs::read(&path)
            .await
            .with_context(|| "Failed to read JSON from file")?;

        let obj = serde_json::from_slice::<Self>(&contents)
            .with_context(|| "Failed to deserialize JSON")?;

        Ok(obj)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub modules: ProjectModulesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectModulesConfig {
    pub pattern: PathBuf,
}

impl ProjectConfig {}

impl SaveAsJson for ProjectConfig {}
impl LoadFromJson for ProjectConfig {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub version: Version,
}

impl ModuleConfig {
    pub fn default() -> Self {
        ModuleConfig {
            version: Version::new(0, 0, 1),
        }
    }
}

impl SaveAsJson for ModuleConfig {}
impl LoadFromJson for ModuleConfig {}
