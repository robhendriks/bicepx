use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// TODO: use method
#[allow(dead_code)]
pub trait Load: for<'de> Deserialize<'de> {
    async fn load_json(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let contents = tokio::fs::read(path)
            .await
            .with_context(|| format!("Failed to read JSON file: {}", path.display()))?;

        let result = serde_json::from_slice::<Self>(&contents)
            .with_context(|| format!("Failed to deserialize JSON file: {}", path.display()))?;

        Ok(result)
    }
}

pub trait Save: Serialize {
    async fn save_json(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        let contents = serde_json::to_vec_pretty(&self)
            .with_context(|| format!("Failed to serialize JSON file: {}", path.display()))?;

        tokio::fs::write(path, contents)
            .await
            .with_context(|| format!("Failed to write JSON file: {}", path.display()))?;

        Ok(())
    }
}
