use super::*;
use futures::Future;

#[derive(Debug, Default)]
pub struct DefaultCacheHandler;

impl ModuleProvider for DefaultCacheHandler {
    fn name(&self) -> &'static str {
        "cache_response"
    }

    fn build(&self, _: ModuleType, _: &unstructured::Document) -> Result<Module> {
        Ok(DefaultCacheHandler.into_module())
    }
}

impl CacheHandlerModule for DefaultCacheHandler {
    fn check_cache(&self, guard: RequestContext) -> ModuleResult {
        let katalyst = ensure!(:guard.katalyst());
        let metadata = ensure!(:guard.metadata());
        if let Ok(instance) = katalyst.get_instance() {
            let cache = instance.service.cache.clone();
            Box::new(cache.get_key(metadata.url.as_str()).then(move |r| match r {
                Ok(r) => {
                    guard.set_http_request(r.as_ref().clone().into_response())?;
                    Err(GatewayError::Done)
                }
                Err(_) => Ok(()),
            }))
        } else {
            Ok(()).fut()
        }
    }

    fn update_cache(&self, guard: RequestContext) -> ModuleResult {
        if !ensure!(:guard.is_response()) {
            return Ok(()).fut();
        }
        let instance = ensure!(:ensure!(:guard.katalyst()).get_instance());
        let cache = instance.service.cache.clone();
        Box::new(guard.preload().and_then(move |_| {
            let req = guard.take_http_request()?;
            guard.set_http_request(match req {
                HttpRequest::LoadedResponse(_) => {
                    cache.set_key(
                        guard.metadata()?.url.as_str(),
                        CachedObject::from_response(&req).unwrap(),
                    );
                    req
                }
                _ => req,
            })?;
            Ok(())
        }))
    }
}
