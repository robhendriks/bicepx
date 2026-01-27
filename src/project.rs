use anyhow::Context;
use glob::glob;
use std::path::PathBuf;

use crate::{
    cli::Ctx,
    config::{LoadFromJson, ModuleConfig, ProjectConfig},
};

pub struct Project {
    pub root: PathBuf,
    pub config: ProjectConfig,
    pub modules: Vec<Module>,
    module_root_paths: Vec<PathBuf>,
}

impl Project {
    pub async fn from_ctx(ctx: &Ctx) -> anyhow::Result<Self> {
        Ok(Project {
            root: ctx.working_dir.clone(),
            config: ProjectConfig::load_from_json(&ctx.config_path).await?,
            modules: Vec::new(),
            module_root_paths: Vec::new(),
        })
    }

    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.discover_module_root_paths()?;
        self.init_modules().await
    }

    fn discover_module_root_paths(&mut self) -> anyhow::Result<()> {
        self.module_root_paths.clear();

        let module_pattern = self.root.join(&self.config.modules.pattern);
        let module_pattern_str = module_pattern
            .to_str()
            .with_context(|| "Failed to construct module pattern")?;

        let module_paths = glob(module_pattern_str)?.filter_map(|e| e.ok());

        for module_path in module_paths {
            let module_root = module_path
                .parent()
                .with_context(|| "Failed to resolve module root path")?;

            self.module_root_paths.push(module_root.to_path_buf());
        }

        Ok(())
    }

    async fn init_modules(&mut self) -> anyhow::Result<()> {
        self.modules.clear();

        for module_root_path in &self.module_root_paths {
            let module_config_path = module_root_path.join("module.json");

            self.modules.push(Module {
                root: module_root_path.clone(),
                config: ModuleConfig::load_from_json(module_config_path).await?,
            });
        }

        Ok(())
    }
}

pub struct Module {
    pub root: PathBuf,
    pub config: ModuleConfig,
}
