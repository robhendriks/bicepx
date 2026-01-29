use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};

use crate::{cli::Cli, console, model};

#[derive(Debug, Args)]
pub struct ShowArgs {
    #[command(subcommand)]
    command: ShowCommands,

    #[arg(short, long, default_value_t = false, global = true)]
    pretty: bool,
}

pub async fn exec(cli: &Cli, args: &ShowArgs) -> Result<()> {
    let project_rc = model::Project::load(&cli.root).await?;
    let project: std::cell::Ref<'_, model::Project> = project_rc.borrow();

    match &args.command {
        ShowCommands::Module(sub_args) => {
            let module = project.find_module(&sub_args.module_name);

            match module {
                Some(module) => {
                    let json = module.to_json();
                    console::write_json(&json, args.pretty)
                }
                None => Err(anyhow!("Module not found: {}", sub_args.module_name)),
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
