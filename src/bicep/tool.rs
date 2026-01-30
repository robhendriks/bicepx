use std::{env, path::PathBuf};

use anyhow::Result;

#[derive(Debug)]
pub struct Cli {
    binary_path: PathBuf,
}

#[cfg(windows)]
const BICEP_BINARY: &'static str = "bicep.exe";
#[cfg(unix)]
const BICEP_BINARY: &'static str = "bicep";
const AZ_HOME: &'static str = ".azure";

impl Cli {
    pub fn new() -> Option<Self> {
        Cli::locate().map(|binary_path| Cli { binary_path })
    }

    pub async fn version(&self) -> Result<String> {
        let output = tokio::process::Command::new(self.binary_path.as_os_str())
            .args(["--version"])
            .output()
            .await?;

        Ok(String::from_utf8(output.stdout)?)
    }

    fn locate() -> Option<PathBuf> {
        let home = env::home_dir()?;
        let bicep_bin = home.join(&AZ_HOME).join("bin").join(&BICEP_BINARY);
        bicep_bin.exists().then(|| bicep_bin)
    }
}
