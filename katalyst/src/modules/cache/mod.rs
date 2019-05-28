mod cache_handler;
mod memory;

use crate::modules::*;
use std::sync::Arc;

pub use cache_handler::DefaultCacheHandler;
pub use memory::MemoryCacheBuilder;

pub fn default_cache() -> Arc<CacheProviderModule> {
    Arc::new(memory::MemoryCache::default())
}
