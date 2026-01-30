use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use json_comments::StripComments;
use serde::Deserialize;

pub mod tool;

#[derive(Debug)]
pub struct Ctx {
    pub config: Cfg,
}

impl Ctx {
    pub async fn init(root: impl AsRef<Path>) -> Result<Ctx> {
        let file = root.as_ref().join("bicepconfig.json");

        Ok(Ctx {
            config: Cfg::from_file(file).await?,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cfg {
    pub module_aliases: ModuleAliases,
}

impl Cfg {
    async fn from_file(file: impl AsRef<Path>) -> Result<Self> {
        let data = tokio::fs::read(&file.as_ref())
            .await
            .with_context(|| "Failed to read bicepconfig.json")?;

        let json_without_comments = StripComments::new(data.as_slice());

        let config: Self = serde_json::from_reader(json_without_comments)
            .with_context(|| "Failed to deserialize bicepconfig.json")?;

        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleAliases {
    pub br: HashMap<String, RegistryAlias>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryAlias {
    pub registry: Option<String>,
    pub module_path: Option<String>,
}
