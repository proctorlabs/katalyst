use crate::prelude::*;
use crate::*;
use futures::future::*;

pub fn authenticate(ctx: Context) -> ModuleResult {
    let route = try_fut!(ctx, ctx.detail.route());
    match &route.authenticators {
        Some(state_authenticators) => {
            let authenticators = state_authenticators.clone();
            let mut result: ModuleResult = ok!(ctx);
            for a in authenticators.iter() {
                result = Box::new(result.and_then({
                    let r = a.clone();
                    move |s| r.dispatch(s)
                }));
            }
            result
        }
        None => ok!(ctx),
    }
}

pub fn authorize(ctx: Context) -> ModuleResult {
    let route = try_fut!(ctx, ctx.detail.route()).clone();
    let mut result: ModuleResult = ok!(ctx);
    if let Some(authorizers) = &route.authorizers {
        for auth in authorizers.iter() {
            let a = auth.clone();
            result = Box::new(result.and_then(move |ctx| a.dispatch(ctx)));
        }
    }
    result
}
