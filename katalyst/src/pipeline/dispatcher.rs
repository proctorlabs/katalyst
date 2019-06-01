use crate::{prelude::*, *};
use futures::future::*;

pub fn run_plugins(ctx: Context) -> ModuleResult {
    let route = try_fut!(ctx, ctx.get_matched()).route.clone();
    let mut result: ModuleResult = ok!(ctx);
    if let Some(plugins) = &route.plugins {
        for plugin in plugins.iter() {
            let p = plugin.clone();
            result = Box::new(result.and_then(move |ctx| p.run(ctx)));
        }
    }
    result
}

pub fn run_handler(ctx: Context) -> ModuleResult {
    let r = try_fut!(ctx, ctx.get_matched()).route.clone();
    Box::new(r.handler.dispatch(ctx))
}
