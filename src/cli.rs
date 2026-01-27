use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::{init::InitArgs, module::ModuleArgs};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(long, global = true, env = "BICEPX_WORKING_DIR", default_value = ".")]
    pub working_dir: PathBuf,

    #[arg(
        long,
        global = true,
        env = "BICEPX_CONFIG",
        default_value = "bicepx.json"
    )]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub async fn exec(&self) -> anyhow::Result<()> {
        let ctx = Ctx::from_cli(&self);

        match &self.command {
            Commands::Init(args) => args.exec(&ctx).await,
            Commands::Module(args) => args.exec(&ctx).await,
        }
    }

    fn get_config_path(&self) -> PathBuf {
        if self.config.is_absolute() {
            self.config.clone()
        } else {
            self.working_dir.join(&self.config)
        }
    }
}

#[derive(Debug)]
pub struct Ctx {
    pub working_dir: PathBuf,
    pub config_path: PathBuf,
}

impl Ctx {
    fn from_cli(cli: &Cli) -> Self {
        Ctx {
            working_dir: cli.working_dir.clone(),
            config_path: cli.get_config_path(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(InitArgs),
    Module(ModuleArgs),
}
