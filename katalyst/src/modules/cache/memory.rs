use super::*;
use futures::future::*;
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

#[derive(Default, Debug)]
pub struct MemoryCacheBuilder;

impl ModuleProvider for MemoryCacheBuilder {
    fn name(&self) -> &'static str {
        "memory_cache"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(MemoryCache::default().into_module())
    }
}

#[derive(Default, Debug)]
pub struct MemoryCache {
    cache: Mutex<HashMap<String, Arc<CachedObject>>>,
}

impl CacheProviderModule for MemoryCache {
    fn get_key(
        &self,
        key: &str,
    ) -> Box<Future<Item = Arc<CachedObject>, Error = GatewayError> + Send> {
        let cache = &self.cache.lock();
        match cache.get(key) {
            Some(r) => Box::new(ok(r.clone())),
            None => fail!(:NOT_FOUND),
        }
    }

    fn set_key(
        &self,
        key: &str,
        val: CachedObject,
    ) -> Box<Future<Item = (), Error = GatewayError> + Send> {
        let cache = &mut self.cache.lock();
        cache.insert(key.to_owned(), Arc::new(val));
        Box::new(ok(()))
    }
}
