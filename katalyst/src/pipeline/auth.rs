use crate::{prelude::*, *};
use futures::future::*;

pub fn authenticate(ctx: Context) -> ModuleResult {
    let matched = try_fut!(ctx, ctx.get_matched());
    let route = &matched.route;
    match &route.authenticators {
        Some(state_authenticators) => {
            let authenticators = state_authenticators.clone();
            let mut result: ModuleResult = ok!(ctx);
            for a in authenticators.iter() {
                result = Box::new(result.and_then({
                    let r = a.clone();
                    move |s| r.authenticate(s)
                }));
            }
            result
        }
        None => ok!(ctx),
    }
}

pub fn authorize(ctx: Context) -> ModuleResult {
    let route = &try_fut!(ctx, ctx.get_matched()).route.clone();
    let mut result: ModuleResult = ok!(ctx);
    if let Some(authorizers) = &route.authorizers {
        for auth in authorizers.iter() {
            let a = auth.clone();
            result = Box::new(result.and_then(move |ctx| a.authorize(ctx)));
        }
    }
    result
}
