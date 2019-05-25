mod definitions;
use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub mod authentication;
pub mod authorization;
pub mod cache;
pub mod errors;
pub mod handlers;
pub mod plugins;
pub use authentication::AuthenticatorModule;
pub use authorization::AuthorizerModule;
pub use cache::{CacheHandler, CacheModule};
pub use definitions::*;
pub use errors::*;
pub use handlers::HandlerModule;
pub use plugins::PluginModule;

pub trait ModuleProviderDefinition {
    const MODULE_TYPE: ModuleType;
    type ModuleImplType;
}


#[derive(Debug)]
pub struct ModuleRegistry {
    modules: HashMap<String, Arc<ModuleProvider>>,
}

impl ModuleRegistry {
    pub fn register(&mut self, module: Arc<ModuleProvider>) {
        self.modules.insert(module.name().to_string(), module);
    }

    pub fn get(&self, name: &str) -> Result<Arc<ModuleProvider>> {
        Ok(self
            .modules
            .get(name)
            .ok_or_else(|| GatewayError::FeatureUnavailable)?
            .clone())
    }
}

macro_rules! register_modules {
    ($($toreg:expr);*) => {
        impl Default for ModuleRegistry {
            fn default() -> Self {
                let mut result = ModuleRegistry {
                    modules: HashMap::default(),
                };
                $(
                    result.register(Arc::new($toreg));
                )*
                result
            }
        }
    };
}

register_modules! {
    handlers::FileServerModule;
    handlers::HostModule;
    authentication::AlwaysAuthenticatorBuilder;
    authentication::NeverAuthenticatorBuilder;
    authentication::HttpAuthenticatorBuilder;
    authentication::WhitelistBuilder;
    plugins::ContentPlugin;
    cache::DefaultCacheHandler;
    cache::MemoryCacheBuilder
}
