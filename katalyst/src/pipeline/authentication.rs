use crate::prelude::*;
use crate::*;
use futures::future::*;

pub fn authenticate(ctx: Context) -> ModuleResult {
    let route = try_fut!(ctx.detail.route());
    match &route.authenticators {
        Some(state_authenticators) => {
            let authenticators = state_authenticators.clone();
            let mut result: ModuleResult = Box::new(ok(ctx));
            for a in authenticators.iter() {
                result = Box::new(result.and_then({
                    let r = a.clone();
                    move |s| r.dispatch(s)
                }));
            }
            result
        }
        None => Box::new(ok(ctx)),
    }
}
