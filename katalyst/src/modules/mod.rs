pub mod authentication;
mod definitions;
pub mod handlers;
pub mod plugins;

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

    pub fn get(&self, name: &str, module_type: &str) -> Result<Arc<Module>, KatalystError> {
        let key = format!("{id}-{mtype}", id = name, mtype = module_type);
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
            .register(Arc::new(authentication::AlwaysAuthenticatorBuilder {}))
            .unwrap();
        result
            .register(Arc::new(authentication::NeverAuthenticatorBuilder {}))
            .unwrap();
        result
            .register(Arc::new(authentication::HttpAuthenticatorBuilder {}))
            .unwrap();
        result
            .register(Arc::new(authentication::WhitelistBuilder {}))
            .unwrap();
        result
            .register(Arc::new(plugins::ContentPlugin {}))
            .unwrap();
        result
    }
}
