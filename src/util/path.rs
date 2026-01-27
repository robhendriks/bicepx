use std::path::Path;

#[allow(dead_code)]
pub trait PathExt {
    fn strip_suffix(&self, suffix: &Path) -> Option<&Path>;
}

impl PathExt for Path {
    fn strip_suffix(&self, suffix: &Path) -> Option<&Path> {
        let mut result = self;

        for _ in 0..suffix.components().count() {
            result = result.parent()?;
        }

        Some(result)
    }
}
