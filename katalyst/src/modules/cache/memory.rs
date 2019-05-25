use super::*;
use futures::future::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Default, Debug)]
pub struct MemoryCacheBuilder;

impl ModuleProvider for MemoryCacheBuilder {
    fn name(&self) -> &'static str {
        "memory_cache"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(Module::CacheProvider(CacheProviderModule(Arc::new(MemoryCache::default()))))
    }
}

#[derive(Default, Debug)]
pub struct MemoryCache {
    cache: RwLock<HashMap<String, Arc<Vec<u8>>>>,
}

impl CacheProvider for MemoryCache {
    fn get_key(&self, key: &str) -> Box<Future<Item = Arc<Vec<u8>>, Error = GatewayError> + Send> {
        Box::new(match self.cache.read() {
            Ok(read) => match read.get(key) {
                Some(r) => ok(r.clone()),
                None => err(GatewayError::StateUnavailable),
            },
            Err(_) => err(GatewayError::StateUnavailable),
        })
    }

    fn set_key(
        &self,
        key: &str,
        val: Vec<u8>,
    ) -> Box<Future<Item = (), Error = GatewayError> + Send> {
        let mut cache = match self.cache.write() {
            Ok(s) => s,
            Err(_) => return Box::new(err(GatewayError::StateUnavailable)),
        };
        cache.insert(key.to_owned(), Arc::new(val));
        Box::new(ok(()))
    }
}
