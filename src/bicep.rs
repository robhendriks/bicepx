pub struct Project {
    modules: Vec<Module>,
}

impl Project {
    pub fn new() -> Self {
        Project {
            modules: Vec::new(),
        }
    }
}

pub struct Module {}
