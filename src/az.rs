use anyhow::{Context, Result};
use serde::Deserialize;

use std::process::Command;

#[derive(Deserialize)]
pub struct AzVersion {
    #[serde(rename = "azure-cli")]
    pub cli: String,
}

pub struct AzCli {}

impl AzCli {
    pub fn get_version() -> Result<AzVersion> {
        let output = Command::new("az")
            .arg("version")
            .arg("--output")
            .arg("json")
            .output()
            .with_context(|| "Failed to execute 'az version' command")?;

        let stdout_str: String = String::from_utf8(output.stdout)
            .with_context(|| "Failed to construct string from 'az version' output")?;

        serde_json::from_str::<AzVersion>(&stdout_str)
            .with_context(|| "Failed to deserialize 'az version' JSON output")
    }

    pub fn get_bicep_version() -> Result<String> {
        let output = Command::new("az")
            .arg("bicep")
            .arg("version")
            .output()
            .with_context(|| "Failed to execute 'az bicep version' command")?;

        let version = String::from_utf8(output.stdout)
            .with_context(|| "Failed to construct string from 'az bicep version' output")?
            .trim()
            .replace("Bicep CLI version ", "")
            .to_string();

        Ok(version)
    }
}
