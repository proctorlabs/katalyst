use crate::prelude::*;
use futures::future::*;

pub fn run_plugins(guard: ContextGuard) -> ModuleResult {
    let mut result: ModuleResult = Ok(()).fut();
    if let Ok(matched) = guard.get_matched() {
        let route = matched.route.clone();
        if let Some(plugins) = &route.plugins {
            for plugin in plugins.iter() {
                let p = plugin.clone();
                let module_guard = guard.clone();
                result = Box::new(result.and_then(move |_| p.run(module_guard)));
            }
        }
    }
    result
}

pub fn run_handler(guard: ContextGuard) -> ModuleResult {
    let matched = ensure_fut!(guard.get_matched());
    let route = matched.route.clone();
    route.handler.dispatch(guard.clone())
}
