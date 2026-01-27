use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::{init::InitArgs, module::ModuleArgs};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(
        short,
        long,
        global = true,
        env = "BICEPX_WORKING_DIR",
        default_value = "."
    )]
    pub working_dir: PathBuf,

    #[arg(
        short,
        long,
        global = true,
        env = "BICEPX_CONFIG",
        default_value = "bicepx.json"
    )]
    pub config: PathBuf,

    #[arg(long, global = true, default_value_t = false)]
    pub no_interact: bool,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub async fn exec(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Init(args) => args.exec(&self).await,
            Commands::Module(args) => args.exec(&self).await,
        }
    }

    pub fn get_config_path(&self) -> PathBuf {
        if self.config.is_absolute() {
            self.config.clone()
        } else {
            self.working_dir.join(&self.config)
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(InitArgs),
    Module(ModuleArgs),
}
