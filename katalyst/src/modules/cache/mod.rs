mod memory;

use crate::modules::*;
use futures::Future;
use std::sync::Arc;

#[derive(Default, Clone, Debug)]
pub struct CacheModule {}
impl PhantomModuleData for CacheModule {
    const MODULE_TYPE: ModuleType = ModuleType::Cache;
    type ModuleImpl = Arc<CacheProvider>;
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
