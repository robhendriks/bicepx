use std::path::Path;

use anyhow::{Context, anyhow};
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

#[allow(dead_code)]
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
pub struct RootConfig {
    pub module_entrypoint: String,
}

impl SaveAsJson for RootConfig {}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleConfig {}

#[allow(dead_code)]
impl ModuleConfig {
    pub fn new() -> Self {
        ModuleConfig {}
    }
}

impl SaveAsJson for ModuleConfig {}
