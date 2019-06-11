use crate::prelude::*;
use futures::future::*;

pub fn authenticate(guard: RequestContext) -> AsyncResult<()> {
    let route = ensure!(:guard.get_route());
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

pub fn authorize(guard: RequestContext) -> AsyncResult<()> {
    let route = ensure!(:guard.get_route());
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
