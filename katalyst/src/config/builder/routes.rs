use super::*;
use crate::{app::Katalyst, instance::Route, modules::*};
use http::Method;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, string::String, sync::Arc};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteBuilder {
    path: PathBuilder,
    #[serde(default)]
    children: Option<Vec<RouteBuilder>>,
    handler: ModuleBuilder<RequestHandler>,
    #[serde(default)]
    methods: Option<Vec<String>>,
    #[serde(default)]
    plugins: Option<Vec<ModuleBuilder<Plugin>>>,
    #[serde(default)]
    cache: Option<ModuleBuilder<CacheHandler>>,
    #[serde(default)]
    authorizers: Option<Vec<ModuleBuilder<Authorizer>>>,
    #[serde(default)]
    authenticators: Option<Vec<ModuleBuilder<Authenticator>>>,
}

impl Builder<Route> for RouteBuilder {
    fn build(&self, engine: Arc<Katalyst>) -> Result<Route> {
        let routebuilders: &Option<Vec<RouteBuilder>> = &self.children;
        let routes = match routebuilders {
            Some(b) => {
                let mut result = vec![];
                for rb in b {
                    result.push(Arc::new(rb.build(engine.clone())?));
                }
                Some(result)
            }
            None => None,
        };
        let handler = module_unwrap!(RequestHandler, self.handler.build(engine.clone())?);

        //Build method hashset
        let methods = match &self.methods {
            Some(s) => {
                let mut vec_methods: HashSet<Method> = HashSet::new();
                for method_string in s {
                    let method = Method::from_bytes(method_string.to_uppercase().as_bytes())?;
                    vec_methods.insert(method);
                }
                Some(vec_methods)
            }
            None => None,
        };

        let plugins = match &self.plugins {
            Some(plugins) => {
                let mut vec_plugins: Vec<Arc<Plugin>> = vec![];
                for p in plugins {
                    vec_plugins.push(module_unwrap!(Plugin, p.build(engine.clone())?));
                }
                Some(vec_plugins)
            }
            None => None,
        };

        let authorizers = match &self.authorizers {
            Some(auths) => {
                let mut vec_auths: Vec<Arc<Authorizer>> = vec![];
                for a in auths {
                    vec_auths.push(module_unwrap!(Authorizer, a.build(engine.clone())?));
                }
                Some(vec_auths)
            }
            None => None,
        };

        let authenticators = match &self.authenticators {
            Some(auths) => {
                let mut vec_auths: Vec<Arc<Authenticator>> = vec![];
                for a in auths {
                    vec_auths.push(module_unwrap!(Authenticator, a.build(engine.clone())?));
                }
                Some(vec_auths)
            }
            None => None,
        };

        let cache: Option<Arc<CacheHandler>> = match &self.cache {
            Some(c) => Some(module_unwrap!(CacheHandler, c.build(engine.clone())?)),
            None => None,
        };

        Ok(Route {
            pattern: Regex::new(&self.path.build(engine)?)?,
            children: routes,
            handler,
            plugins,
            authorizers,
            cache,
            methods,
            authenticators,
        })
    }
}
