use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{cli::Cli, console, model};

#[derive(Debug, Args)]
pub struct ListArgs {
    #[command(subcommand)]
    command: ListCommands,

    #[arg(short, long, default_value_t = false, global = true)]
    pretty: bool,
}

#[derive(Debug, Subcommand)]
enum ListCommands {
    #[command(alias = "mod")]
    Module,
}

pub async fn exec(cli: &Cli, args: &ListArgs) -> Result<()> {
    let project_rc = model::Project::load(&cli.root).await?;

    match &args.command {
        ListCommands::Module => {
            let project = project_rc.borrow();

            let json: Vec<model::ModuleJson> =
                project.modules.iter().map(|m| m.to_json()).collect();

            console::write_json(&json, args.pretty)
        }
    }
}
