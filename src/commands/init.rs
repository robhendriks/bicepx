use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use glob::glob;
use log::info;

use crate::{
    cli::Ctx,
    config::{ModuleConfig, ProjectConfig, ProjectModulesConfig, SaveAsJson},
};

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(long, default_value_t = false)]
    overwrite: bool,

    #[arg(long, default_value = "*.bicep")]
    pattern: PathBuf,
}

impl InitArgs {
    pub async fn exec(&self, ctx: &Ctx) -> anyhow::Result<()> {
        self.init_project_config(ctx).await?;
        self.init_module_config(ctx).await?;

        info!("Done");

        Ok(())
    }

    async fn init_project_config(&self, ctx: &Ctx) -> anyhow::Result<()> {
        let config = ProjectConfig {
            modules: ProjectModulesConfig {
                pattern: self.pattern.clone(),
            },
        };

        info!("Creating project config at {}", ctx.config_path.display());

        config.save_as_json(&ctx.config_path, self.overwrite).await
    }

    async fn init_module_config(&self, ctx: &Ctx) -> anyhow::Result<()> {
        let module_pattern = ctx.working_dir.join(&self.pattern);
        let module_pattern_str = module_pattern
            .to_str()
            .with_context(|| "Failed to construct module pattern")?;

        let module_paths = glob(module_pattern_str)?.filter_map(|e| e.ok());

        for module_path in module_paths {
            let module_root_path = module_path
                .parent()
                .with_context(|| "Failed to get module root path")?;

            let module_config_path = module_root_path.join("module.json");

            let module_config = ModuleConfig::default();

            info!("Creating module at {}", module_root_path.display());

            module_config
                .save_as_json(&module_config_path, self.overwrite)
                .await?;
        }

        Ok(())
    }
}
