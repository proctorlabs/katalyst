use super::*;
use crate::app::KatalystEngine;
use crate::error::KatalystError;
use crate::state::Route;
use http::Method;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RouteBuilder {
    pattern: String,
    children: Option<Vec<RouteBuilder>>,
    downstream: DownstreamBuilder,
    methods: Option<Vec<String>>,
    authenticators: Option<Vec<AuthenticatorBuilder>>,
}

impl Builder<Route> for RouteBuilder {
    fn build(&self, engine: Arc<KatalystEngine>) -> Result<Route, KatalystError> {
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
        let downstream = self.downstream.build(engine.clone())?;

        //Build method hashset
        let methods = match &self.methods {
            Some(s) => {
                let mut vec_methods: HashSet<Method> = HashSet::new();
                for method_string in s {
                    let method =
                        Method::from_bytes(method_string.to_uppercase().as_bytes()).unwrap();
                    vec_methods.insert(method);
                }
                Some(vec_methods)
            }
            None => None,
        };

        let authenticators = match &self.authenticators {
            Some(auths) => {
                let mut vec_auths: Vec<Authenticator> = vec![];
                for a in auths {
                    vec_auths.push(a.build(engine.clone())?);
                }
                Some(vec_auths)
            }
            None => None,
        };

        Ok(Route {
            pattern: Regex::new(&self.pattern)?,
            children: routes,
            downstream: downstream,
            methods: methods,
            authenticators: authenticators,
        })
    }
}
