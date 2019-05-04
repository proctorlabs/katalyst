mod definitions;
use crate::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub mod authentication;
pub mod authorization;
pub mod cache;
pub mod errors;
pub mod handlers;
pub mod plugins;
pub use authentication::AuthenticatorModule;
pub use authorization::AuthorizerModule;
pub use cache::CacheProvider;
pub use definitions::*;
pub use errors::*;
pub use handlers::HandlerModule;
pub use plugins::PluginModule;

mod sealed {
    pub trait ModuleMethodImpl {}
}

pub trait PhantomModuleData {
    const MODULE_TYPE: ModuleType;
    type ModuleImpl: sealed::ModuleMethodImpl + Send + Sync + Debug + Sized;
}

pub trait ModuleDispatch: Send + Sync + Debug {
    fn dispatch(&self, ctx: Context) -> ModuleResult;
}
impl sealed::ModuleMethodImpl for Arc<ModuleDispatch> {}
impl sealed::ModuleMethodImpl for Arc<cache::CacheProvider> {}

pub struct Modules {
    modules: HashMap<String, Arc<Module>>,
}

impl Modules {
    pub fn register(&mut self, module: Arc<Module>) -> Result<(), KatalystError> {
        self.modules.insert(module.name().to_string(), module);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<Arc<Module>, KatalystError> {
        Ok(self
            .modules
            .get(name)
            .ok_or_else(|| KatalystError::FeatureUnavailable)?
            .clone())
    }
}

macro_rules! register_modules {
    ($($toreg:expr);*) => {
        impl Default for Modules {
            fn default() -> Self {
                let mut result = Modules {
                    modules: HashMap::default(),
                };
                $(
                    result.register(Arc::new($toreg)).unwrap();
                )*
                result
            }
        }
    };
}

register_modules! {
    handlers::FileServerModule{};
    handlers::HostModule{};
    authentication::AlwaysAuthenticatorBuilder{};
    authentication::NeverAuthenticatorBuilder{};
    authentication::HttpAuthenticatorBuilder{};
    authentication::WhitelistBuilder{};
    plugins::ContentPlugin{}
}
