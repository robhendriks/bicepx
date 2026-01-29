use serde::Serialize;
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
};

use anyhow::Result;

use crate::config::{self, json::Load};

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    #[allow(unused)]
    pub config: config::root::Cfg,
    pub modules: Vec<Module>,
}

impl Project {
    pub fn find_module(&self, name: &str) -> Option<&Module> {
        let lc_name = name.to_lowercase();

        self.modules
            .iter()
            .find(|m| m.config.name.to_lowercase() == lc_name)
    }

    pub async fn load(root: impl AsRef<Path>) -> Result<Rc<RefCell<Project>>> {
        let config_file = config::root::Cfg::build_path(&root);
        let config = config::root::Cfg::load_json(&config_file).await?;

        let project = Rc::new(RefCell::new(Project {
            root: root.as_ref().to_path_buf(),
            config,
            modules: Vec::new(),
        }));

        let module_files = Module::discover_files(&root)?;

        for module_file in module_files {
            let module = Project::load_module(&project, &module_file).await?;
            project.borrow_mut().modules.push(module);
        }

        Ok(project)
    }

    async fn load_module(
        project_rc: &Rc<RefCell<Project>>,
        file: impl AsRef<Path>,
    ) -> Result<Module> {
        let file = file.as_ref();
        let config = config::module::Cfg::load_json(&file).await?;

        Ok(Module {
            project: Rc::downgrade(project_rc),
            root: file.parent().unwrap().to_path_buf(),
            config,
        })
    }
}

#[derive(Debug)]
pub struct Module {
    pub project: Weak<RefCell<Project>>,
    pub root: PathBuf,
    pub config: config::module::Cfg,
}

impl Module {
    pub fn discover_files(root: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
        let root = root.as_ref();
        let pattern = root.join("**").join(config::module::FILE_NAME);

        Ok(glob::glob(pattern.to_str().unwrap())?
            .filter_map(|e| e.ok())
            .collect())
    }

    pub fn to_json(&self) -> ModuleJson {
        let project_rc = self.project.upgrade().unwrap();
        let project = project_rc.borrow();

        ModuleJson {
            path: self.root.strip_prefix(&project.root).unwrap().to_path_buf(),
            main: self.config.main.to_owned(),
            version: self.config.version.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ModuleJson {
    pub path: PathBuf,
    pub main: String,
    pub version: String,
}
