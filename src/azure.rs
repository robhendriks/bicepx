use std::{env, path::PathBuf};

use anyhow::{Context, Ok, Result};
use log::debug;
use serde_json::Value;

pub struct AzCli {}

impl AzCli {
    pub async fn init() -> Result<()> {
        let az_version = AzCli::version()
            .await
            .with_context(|| "Failed to determine Azure CLI version")?;

        debug!("az version: {}", az_version);

        Ok(())
    }

    pub async fn version() -> Result<String> {
        let output = tokio::process::Command::new("az")
            .args(["version", "--output", "json"])
            .output()
            .await
            .with_context(|| "Failed to execute Azure CLI command")?;

        let output_json = serde_json::from_slice::<Value>(&output.stdout)
            .with_context(|| "Failed to parse Azure CLI JSON output")?;

        let version_str = output_json["azure-cli"]
            .as_str()
            .with_context(|| "Failed to get Azure CLI version string")?;

        Ok(String::from(version_str))
    }
}

const BICEP_VERSION_PREFIX: &'static str = "Bicep CLI version";

pub struct BicepCli {}

impl BicepCli {
    fn get_bin_path() -> PathBuf {
        env::home_dir().unwrap().join(".azure/bin/bicep")
    }

    pub async fn init() -> Result<()> {
        let az_bicep_version = BicepCli::version()
            .await
            .with_context(|| "Failed to determine Bicep CLI version")?;

        debug!("az bicep version: {}", az_bicep_version);

        Ok(())
    }

    pub async fn version() -> Result<String> {
        let output = tokio::process::Command::new(BicepCli::get_bin_path())
            .args(["--version"])
            .output()
            .await
            .with_context(|| "Failed to execute Bicep CLI command")?;

        let output_str = String::from_utf8(output.stdout)
            .with_context(|| "Failed to construct string from Bicep CLI output")?;

        let output_str_without_prefix = output_str.replace(BICEP_VERSION_PREFIX, "");

        Ok(String::from(output_str_without_prefix.trim()))
    }
}
