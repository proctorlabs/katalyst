mod cache_handler;
mod memory;

use crate::modules::*;
use futures::Future;
use std::sync::Arc;

pub use cache_handler::DefaultCacheHandler;
pub use memory::MemoryCacheBuilder;

#[derive(Default, Clone, Debug)]
pub struct CacheModule;

impl ModuleProvider for CacheModule {
    const MODULE_TYPE: ModuleType = ModuleType::CacheProvider;

    type ModuleImplType = Arc<CacheProvider>;

    fn build(
        module: Arc<Module>,
        instance: Arc<Katalyst>,
        doc: &unstructured::Document,
    ) -> Result<Self::ModuleImplType> {
        module.build_cache(Self::MODULE_TYPE, instance, doc)
    }
}

#[derive(Default, Clone, Debug)]
pub struct CacheHandler;

impl ModuleProvider for CacheHandler {
    const MODULE_TYPE: ModuleType = ModuleType::CacheHandler;

    type ModuleImplType = Arc<ModuleDispatch>;

    fn build(
        module: Arc<Module>,
        instance: Arc<Katalyst>,
        doc: &unstructured::Document,
    ) -> Result<Self::ModuleImplType> {
        module.build_hook(Self::MODULE_TYPE, instance, doc)
    }
}

pub trait CacheProvider: Send + Sync + Debug {
    fn get_key(&self, key: &str) -> Box<Future<Item = Arc<Vec<u8>>, Error = GatewayError> + Send>;

    fn set_key(
        &self,
        key: &str,
        val: Vec<u8>,
    ) -> Box<Future<Item = (), Error = GatewayError> + Send>;
}

pub fn default_cache() -> Arc<CacheProvider> {
    Arc::new(memory::MemoryCache::default())
}
