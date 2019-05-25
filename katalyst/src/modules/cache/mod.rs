mod cache_handler;
mod memory;

use crate::modules::*;
use std::sync::Arc;

pub use cache_handler::DefaultCacheHandler;
pub use memory::MemoryCacheBuilder;

#[derive(Default, Clone, Debug)]
pub struct CacheModule;

#[derive(Default, Clone, Debug)]
pub struct CacheHandler;

impl ModuleProviderDefinition for CacheModule {
    const MODULE_TYPE: ModuleType = ModuleType::CacheProvider;
    type ModuleImplType = Arc<CacheProvider>;
}

impl ModuleProviderDefinition for CacheHandler {
    const MODULE_TYPE: ModuleType = ModuleType::CacheHandler;
    type ModuleImplType = Arc<ModuleDispatch>;
}

pub fn default_cache() -> Arc<CacheProvider> {
    Arc::new(memory::MemoryCache::default())
}
