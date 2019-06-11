mod cache_handler;
mod memory;

use crate::modules::*;
use hyper::Response;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

pub use cache_handler::DefaultCacheHandler;
pub use memory::MemoryCacheBuilder;

pub fn default_cache() -> Arc<CacheProviderModule + Send> {
    Arc::new(memory::MemoryCache::default())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CachedObject {
    pub content: Vec<u8>,
    pub headers: HashMap<String, String>,
}

impl CachedObject {
    pub fn from_response(req: &HttpRequest) -> Result<Self> {
        match req {
            HttpRequest::LoadedResponse(r) => Ok(CachedObject {
                content: r.1.clone(),
                headers: r
                    .0
                    .headers
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect(),
            }),
            HttpRequest::ParsedResponse(r) => Ok(CachedObject {
                content: r.1.clone(),
                headers: r
                    .0
                    .headers
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect(),
            }),
            _ => fail!(NOT_FOUND),
        }
    }

    pub fn into_response(self) -> HttpRequest {
        let mut builder = Response::builder();
        for (k, v) in self.headers.into_iter() {
            builder.header(k.as_str(), v.as_str());
        }
        let p = builder.body(hyper::Body::empty()).unwrap().into_parts().0;
        HttpRequest::LoadedResponse(Box::new((p, self.content)))
    }
}
