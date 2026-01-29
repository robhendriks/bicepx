use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::command::{
    build::{self, BuildArgs},
    docs::{self, DocsArgs},
    init::{self, InitArgs},
    list::{self, ListArgs},
    show::{self, ShowArgs},
};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long, default_value = ".", env = "BICEPX_ROOT", global = true)]
    pub root: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Init(args) => init::exec(self, args).await,
            Commands::List(args) => list::exec(self, args).await,
            Commands::Show(args) => show::exec(self, args).await,
            Commands::Docs(args) => docs::exec(self, args).await,
            Commands::Build(args) => build::exec(self, args).await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init(InitArgs),
    #[command(alias = "ls")]
    List(ListArgs),
    #[command(alias = "s")]
    Show(ShowArgs),
    #[command(alias = "d")]
    Docs(DocsArgs),
    #[command(alias = "b")]
    Build(BuildArgs),
}
