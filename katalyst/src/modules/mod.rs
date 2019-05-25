mod definitions;
mod registry;
use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
mod authentication;
mod authorization;
mod errors;
mod handlers;
mod plugins;

pub(crate) mod cache;
pub use authentication::AuthenticatorModule;
pub use authorization::AuthorizerModule;
pub use cache::{CacheHandler, CacheModule};
pub use definitions::*;
pub use errors::*;
pub use handlers::HandlerModule;
pub use plugins::PluginModule;
pub use registry::ModuleRegistry;

pub trait ModuleProviderDefinition {
    const MODULE_TYPE: ModuleType;
    type ModuleImplType;
}
