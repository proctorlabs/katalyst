use crate::prelude::*;

pub fn check_cache(guard: RequestContext) -> ModuleResult {
    let route = ensure!(:guard.get_route());
    if let Some(cache) = &route.cache {
        Box::new(cache.check_cache(guard.clone()))
    } else {
        Ok(()).fut()
    }
}

pub fn update_cache(guard: RequestContext) -> ModuleResult {
    let route = ensure!(:guard.get_route());
    if let Some(cache) = &route.cache {
        Box::new(cache.update_cache(guard.clone()))
    } else {
        Ok(()).fut()
    }
}
