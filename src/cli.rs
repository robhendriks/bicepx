use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{init, module};

#[derive(Debug, Parser)]
#[command(name = "bicepx")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        short,
        long,
        global = true,
        default_value = ".",
        env = "BICEPX_WORKING_DIR"
    )]
    pub working_dir: PathBuf,

    #[arg(
        short,
        long,
        global = true,
        default_value = "bicepx.json",
        env = "BICEPX_CONFIG"
    )]
    pub config: PathBuf,
}

impl Cli {
    pub fn get_config_path(&self) -> PathBuf {
        if self.config.is_absolute() {
            self.config.clone()
        } else {
            self.working_dir.join(&self.config)
        }
    }

    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Init(args) => init::execute(self, args).await,
            Commands::Module(args) => module::ModuleArgs::execute(self, args).await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init(init::InitArgs),
    Module(module::ModuleArgs),
}
