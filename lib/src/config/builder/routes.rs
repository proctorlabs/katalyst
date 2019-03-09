use super::downstream::DownstreamBuilder;
use crate::state::Route;
use crate::templates::Providers;
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
}

impl RouteBuilder {
    pub fn build(&self, providers: &Providers) -> Route {
        let routebuilders: &Option<Vec<RouteBuilder>> = &self.children;
        let routes = match routebuilders {
            Some(b) => {
                let mut result = vec![];
                for rb in b {
                    result.push(Arc::new(rb.build(providers)));
                }
                Some(result)
            }
            None => None,
        };
        let downstream = self.downstream.build(providers);

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

        Route {
            pattern: Regex::new(&self.pattern).unwrap(),
            children: routes,
            downstream: downstream,
            methods: methods,
        }
    }
}
