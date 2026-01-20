use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct BicepModule {}

impl BicepModule {
    pub fn discover_module_paths(
        path: impl AsRef<Path>,
        entrypoint: impl AsRef<Path>,
    ) -> Result<Vec<PathBuf>> {
        let mut modules = Vec::new();

        for entry in WalkDir::new(path).follow_links(false) {
            let entry = entry?;
            let path = entry.path();

            if path.file_name() == Some("main.bicep".as_ref()) {
                if path.ends_with(&entrypoint) {
                    modules.push(path.to_path_buf())
                }
            }
        }

        modules.sort();
        Ok(modules)
    }
}
