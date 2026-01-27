use std::path::PathBuf;

use clap::{Args, Subcommand};
use semver::Version;
use serde::Serialize;

use crate::{
    cli::Ctx,
    project::{Module, Project},
};

#[derive(Debug, Args)]
pub struct ModuleArgs {
    #[command(subcommand)]
    command: ModuleCommands,
}

impl ModuleArgs {
    pub async fn exec(&self, ctx: &Ctx) -> anyhow::Result<()> {
        let mut project = Project::from_ctx(&ctx).await?;

        project.init().await?;

        match &self.command {
            ModuleCommands::List => {
                let module_list_json = get_module_list_json(&project);

                println!("{}", serde_json::to_string_pretty(&module_list_json)?)
            }
            ModuleCommands::Show(args) => {
                let module = project.find_module(&args.name);

                match module {
                    Some(module) => {
                        let module_json = get_module_json(module);
                        println!("{}", serde_json::to_string_pretty(&module_json)?);
                    }
                    None => return Err(anyhow::anyhow!("Module not found: {}", args.name)),
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum ModuleCommands {
    #[command(alias = "ls")]
    List,
    Show(ModuleShowArgs),
}

#[derive(Debug, Args)]
pub struct ModuleShowArgs {
    #[arg(short, long)]
    pub name: String,
}

#[derive(Serialize)]
struct ModuleJson<'a> {
    #[serde(borrow)]
    name: &'a str,

    #[serde(borrow)]
    version: &'a Version,

    #[serde(borrow)]
    path: &'a PathBuf,
}

fn get_module_list_json<'a>(project: &'a Project) -> Vec<ModuleJson<'a>> {
    project
        .modules
        .iter()
        .map(|m| ModuleJson {
            name: &m.config.name,
            version: &m.config.version,
            path: &m.root,
        })
        .collect()
}

fn get_module_json<'a>(module: &'a Module) -> ModuleJson<'a> {
    ModuleJson {
        name: &module.config.name,
        version: &module.config.version,
        path: &module.root,
    }
}
