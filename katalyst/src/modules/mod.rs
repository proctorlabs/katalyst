mod definitions;
pub mod handlers;

use crate::prelude::*;
pub use definitions::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Modules {
    modules: HashMap<String, Arc<Module>>,
}

impl Modules {
    pub fn register(&mut self, module: Arc<Module>) -> Result<(), KatalystError> {
        self.modules.insert(module.key(), module);
        Ok(())
    }

    pub fn get(&self, name: &str, module_type: ModuleType) -> Result<Arc<Module>, KatalystError> {
        let key = format!("{id}-{mtype}", id = name, mtype = module_type.type_id());
        Ok(self
            .modules
            .get(&key)
            .ok_or_else(|| KatalystError::FeatureUnavailable)?
            .clone())
    }
}

impl Default for Modules {
    fn default() -> Self {
        let mut result = Modules {
            modules: HashMap::default(),
        };
        result
            .register(Arc::new(handlers::FileServerModule {}))
            .unwrap();
        result.register(Arc::new(handlers::HostModule {})).unwrap();
        result
    }
}
