use std::path::Path;

use anyhow::{Context, Result, anyhow};
use semver::Version;
use serde::{Deserialize, Serialize};

pub trait Config: Serialize + for<'de> Deserialize<'de> {
    async fn load_from(path: impl AsRef<Path>) -> Result<Self> {
        let config_buf = tokio::fs::read(path).await.with_context(|| "")?;

        let config = serde_json::from_slice::<Self>(&config_buf).with_context(|| "")?;

        Ok(config)
    }

    async fn save_to(&self, path: impl AsRef<Path>, overwrite: bool) -> Result<()> {
        let path_ref = path.as_ref();

        if !overwrite && path_ref.exists() {
            return Err(anyhow!("File exists at {}", path_ref.display()));
        }

        let proj_config_buf = serde_json::to_vec_pretty(&self)
            .with_context(|| format!("Failed to serialize JSON {}", path_ref.display()))?;

        tokio::fs::write(path_ref, &proj_config_buf)
            .await
            .with_context(|| format!("Failed to write file {}", path_ref.display()))?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {}

impl ProjectConfig {
    pub fn new() -> Self {
        ProjectConfig {}
    }
}

impl Config for ProjectConfig {}

#[derive(Serialize, Deserialize)]
pub struct ModuleConfig {
    version: Version,
    lint: Option<bool>,
    format: Option<bool>,
}

impl Config for ModuleConfig {}

impl ModuleConfig {
    pub fn new() -> Self {
        ModuleConfig {
            lint: Some(true),
            format: Some(true),
            version: Version::new(0, 0, 1),
        }
    }
}
