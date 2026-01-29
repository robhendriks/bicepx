use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use log::info;

use crate::{cli::Cli, model};

#[derive(Debug, Args)]
pub struct ShowArgs {
    #[command(subcommand)]
    command: ShowCommands,
}

pub async fn exec(cli: &Cli, args: &ShowArgs) -> Result<()> {
    let project_rc = model::Project::load(&cli.root).await?;
    let project: std::cell::Ref<'_, model::Project> = project_rc.borrow();

    match &args.command {
        ShowCommands::Module(args) => {
            let module = project.find_module(&args.module_name);

            match module {
                Some(module) => {
                    info!("{}", module.root.display());
                    Ok(())
                }
                None => Err(anyhow!("Module not found: {}", args.module_name)),
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum ShowCommands {
    #[command(alias = "mod")]
    Module(ModuleArgs),
}

#[derive(Debug, Args)]
struct ModuleArgs {
    module_name: String,
}
