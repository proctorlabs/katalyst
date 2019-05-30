use crate::prelude::*;
use crate::*;

pub fn check_cache(ctx: Context) -> ModuleResult {
    if let Some(cache) = &try_fut!(ctx, ctx.get_matched()).route.clone().cache {
        Box::new(cache.check_cache(ctx))
    } else {
        ok!(ctx)
    }
}

pub fn update_cache(ctx: Context) -> ModuleResult {
    if let Some(cache) = &try_fut!(ctx, ctx.get_matched()).route.clone().cache {
        Box::new(cache.update_cache(ctx))
    } else {
        ok!(ctx)
    }
}
