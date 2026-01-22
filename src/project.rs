use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Project {
    pub root_path: PathBuf,
    pub modules: Vec<Module>,
}

impl Project {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Project {
            root_path: path.as_ref().to_path_buf(),
            modules: Vec::new(),
        }
    }

    pub fn discover_modules(&mut self, pattern: impl AsRef<Path>) -> Result<usize> {
        self.modules.clear();

        for entry in WalkDir::new(&self.root_path).follow_links(false) {
            let entry = entry?;
            let path = entry.path();

            if path.ends_with(&pattern) {
                if let Some(parent) = path.parent() {
                    self.modules.push(Module {
                        root_path: parent.to_path_buf(),
                    })
                }
            }
        }

        Ok(self.modules.len())
    }
}

#[derive(Debug)]
pub struct Module {
    pub root_path: PathBuf,
}
