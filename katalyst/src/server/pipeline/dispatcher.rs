use crate::prelude::*;
use futures::future::*;

pub(crate) fn run_plugins(guard: RequestContext) -> ModuleResult {
    let mut result: ModuleResult = Ok(()).fut();
    let route = ensure!(:guard.get_route());
    if let Some(plugins) = &route.plugins {
        for plugin in plugins.iter() {
            let p = plugin.clone();
            let module_guard = guard.clone();
            result = Box::new(result.and_then(move |_| p.run(module_guard)));
        }
    }
    result
}

pub(crate) fn run_handler(guard: RequestContext) -> ModuleResult {
    let route = ensure!(:guard.get_route());
    route.handler.dispatch(guard.clone())
}
