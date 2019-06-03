use crate::prelude::*;
use futures::future::*;

pub fn authenticate(guard: ContextGuard) -> AsyncResult<()> {
    let matched = ensure_fut!(guard.get_matched());
    let route = &matched.route;
    match &route.authenticators {
        Some(state_authenticators) => {
            let authenticators = state_authenticators.clone();
            let mut result: AsyncResult<()> = Ok(()).fut();
            for authenticator in authenticators.iter() {
                let module_guard = guard.clone();
                result = Box::new(result.and_then({
                    let r = authenticator.clone();
                    move |_| r.authenticate(module_guard)
                }));
            }
            result
        }
        None => Ok(()).fut(),
    }
}

pub fn authorize(guard: ContextGuard) -> AsyncResult<()> {
    let route = &ensure_fut!(guard.get_matched()).route.clone();
    let mut result: AsyncResult<()> = Ok(()).fut();
    if let Some(authorizers) = &route.authorizers {
        for auth in authorizers.iter() {
            let a = auth.clone();
            let module_guard = guard.clone();
            result = Box::new(result.and_then(move |_| a.authorize(module_guard)));
        }
    }
    result
}
