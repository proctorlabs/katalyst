use crate::prelude::*;

pub fn check_cache(guard: ContextGuard) -> ModuleResult {
    if let Some(cache) = &ensure_fut!(guard.get_matched()).route.clone().cache {
        Box::new(cache.check_cache(guard.clone()))
    } else {
        Ok(()).fut()
    }
}

pub fn update_cache(guard: ContextGuard) -> ModuleResult {
    if let Some(cache) = &ensure_fut!(guard.get_matched()).route.clone().cache {
        Box::new(cache.update_cache(guard.clone()))
    } else {
        Ok(()).fut()
    }
}
