use anyhow::{Context, Result};
use regex::Regex;
use semver::Version;
use serde::Deserialize;
use tokio::process::Command;

use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Deserialize)]
pub struct AzVersion {
    #[serde(rename = "azure-cli")]
    pub cli: Version,
}

pub struct AzCli {}

impl AzCli {
    pub async fn get_version() -> Result<AzVersion> {
        // Execute command
        let output = Command::new("az")
            .arg("version")
            .arg("--output")
            .arg("json")
            .output()
            .await
            .with_context(|| "Failed to execute 'az version' command")?;

        // Get stdout as string
        let stdout_str: String = String::from_utf8(output.stdout)
            .with_context(|| "Failed to construct string from 'az version' output")?;

        // Parse the JSON output
        serde_json::from_str::<AzVersion>(&stdout_str)
            .with_context(|| "Failed to deserialize 'az version' JSON output")
    }

    pub async fn get_bicep_version() -> Result<Version> {
        // Execute command
        let output = Command::new("az")
            .arg("bicep")
            .arg("version")
            .output()
            .await
            .with_context(|| "Failed to execute 'az bicep version' command")?;

        // Get stdout as string
        let stdout_str = String::from_utf8(output.stdout)
            .with_context(|| "Failed to construct string from 'az bicep version' output")?;

        // The command 'az bicep version' outputs 'Bicep CLI version x.x.x (xxx)'
        // Use a regex to extract the semver portion we need (x.x.x)
        let semver_re = Regex::new(r"([0-9]+\.[0-9]+\.[0-9]+)").unwrap();
        let semver_re_captures = semver_re
            .captures(&stdout_str)
            .with_context(|| "Failed to parse 'az bicep version' output")?;

        // Get first capture and construct semver object
        let version_str = semver_re_captures.get(1).unwrap().as_str();
        let version = Version::from_str(&version_str).unwrap();

        Ok(version)
    }

    pub async fn compile_module(file: impl AsRef<Path>) -> Result<(PathBuf, Vec<u8>)> {
        let output = Command::new("az")
            .arg("bicep")
            .arg("build")
            .arg("--file")
            .arg(file.as_ref())
            .arg("--stdout")
            .output()
            .await
            .with_context(|| format!("Failed to build Bicep file {}", file.as_ref().display()))?;

        Ok((file.as_ref().to_path_buf(), output.stdout.clone()))
    }
}
