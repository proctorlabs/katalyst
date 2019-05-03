mod memory;

use crate::prelude::*;
use futures::Future;
use std::sync::Arc;

pub trait CacheProvider {
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
