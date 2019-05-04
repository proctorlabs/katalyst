mod memory;

use crate::modules::*;
use futures::Future;
use std::sync::Arc;

pub use memory::MemoryCacheBuilder;

#[derive(Default, Clone, Debug)]
pub struct CacheModule {}

impl ModuleProvider for CacheModule {
    const MODULE_TYPE: ModuleType = ModuleType::Cache;

    type ModuleImplType = Arc<CacheProvider>;

    fn build(
        module: Arc<Module>,
        instance: Arc<Katalyst>,
        doc: &unstructured::Document,
    ) -> Result<Self::ModuleImplType, ConfigurationFailure> {
        module.build_cache(Self::MODULE_TYPE, instance, doc)
    }
}

pub trait CacheProvider: Send + Sync + Debug {
    fn get_key(&self, key: &str) -> Box<Future<Item = Arc<Vec<u8>>, Error = KatalystError> + Send>;

    fn set_key(
        &mut self,
        key: &str,
        val: Vec<u8>,
    ) -> Box<Future<Item = (), Error = KatalystError> + Send>;
}

pub fn default_cache() -> Box<CacheProvider> {
    Box::new(memory::MemoryCache::default())
}
