use super::*;
use futures::future::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct MemoryCache {
    cache: HashMap<String, Arc<Vec<u8>>>,
}

impl CacheProvider for MemoryCache {
    fn get_key(&self, key: &str) -> Box<Future<Item = Arc<Vec<u8>>, Error = KatalystError> + Send> {
        Box::new(match self.cache.get(key) {
            Some(r) => ok(r.clone()),
            None => err(KatalystError::StateUnavailable),
        })
    }

    fn set_key(
        &mut self,
        key: &str,
        val: Vec<u8>,
    ) -> Box<Future<Item = (), Error = KatalystError> + Send> {
        self.cache.insert(key.to_owned(), Arc::new(val));
        Box::new(ok(()))
    }
}
