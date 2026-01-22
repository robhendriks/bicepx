use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

pub trait Config: Serialize + for<'de> Deserialize<'de> {
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
    main: PathBuf,
}

impl Config for ModuleConfig {}

impl ModuleConfig {
    pub fn new(main: impl AsRef<Path>) -> Self {
        ModuleConfig {
            main: main.as_ref().to_path_buf(),
        }
    }
}
