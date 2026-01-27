use anyhow::Context;
use clap::Args;
use inquire::Text;
use log::info;

use crate::{
    cli::Cli,
    config::{RootConfig, SaveAsJson},
};

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false)]
    force: bool,

    #[arg(short, long, default_value = "bicep/main.bicep")]
    module_entrypoint: String,
}

impl InitArgs {
    pub async fn exec(&self, cli: &Cli) -> anyhow::Result<()> {
        let module_entrypoint = cli.prompt_or(
            || Text::new("Module entrypoint").with_default("bicep/main.bicep"),
            || self.module_entrypoint.clone(),
        )?;

        let config = RootConfig { module_entrypoint };
        let config_path = cli.get_config_path();

        info!("Creating root config: {}", config_path.display());

        config
            .save_as_json(&config_path, self.force)
            .await
            .with_context(|| "Failed to create root config")?;

        Ok(())
    }
}
