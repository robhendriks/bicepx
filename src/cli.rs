use std::path::PathBuf;

use clap::{Parser, Subcommand};
use inquire::Text;

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

    pub fn prompt_or<'a, 'b, PF, VF>(
        &self,
        prompt_factory: PF,
        value_factory: VF,
    ) -> anyhow::Result<String>
    where
        PF: Fn() -> Text<'a, 'b>,
        VF: Fn() -> String,
    {
        if self.no_interact {
            Ok(value_factory())
        } else {
            let prompt = prompt_factory();
            Ok(prompt.prompt()?)
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(InitArgs),
    Module(ModuleArgs),
}
