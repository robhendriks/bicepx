use anyhow::{Context, Ok};
use clap::Args;
use log::info;

use crate::{
    cli::Cli,
    config::{RootConfig, SaveAsJson},
};

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

impl InitArgs {
    pub async fn exec(&self, _cli: &Cli) -> anyhow::Result<()> {
        let config = RootConfig::new();
        let config_path = _cli.get_config_path();

        info!("Creating root config: {}", config_path.display());

        config
            .save_as_json(&config_path, self.force)
            .await
            .with_context(|| "Failed to create root config")?;

        Ok(())
    }
}
