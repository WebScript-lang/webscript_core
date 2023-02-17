use std::{collections::HashMap, sync::RwLock};

use super::module::Module;

pub struct Environment(RwLock<EnvironmentData>);

pub struct EnvironmentData {
    modules: HashMap<String, Module>,
}

impl Environment {
    pub fn new() -> Self {
        Self(RwLock::new(EnvironmentData {
            modules: HashMap::new(),
        }))
    }

    pub fn add_module(&self, module: Module) {
        let path = module.path();
        let modules = &mut self.0.write().unwrap().modules;
        let exists = modules.insert(path.clone(), module).is_some();

        if exists {
            panic!("Module \"{path}\" already exists");
        }
    }
}
