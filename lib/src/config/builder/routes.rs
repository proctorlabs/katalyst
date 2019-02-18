use super::downstream::DownstreamBuilder;
use crate::config::Route;
use http::Method;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;
use std::string::String;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RouteBuilder<'a> {
    pattern: RefCell<Option<String>>,
    children: RefCell<Option<Vec<RouteBuilder<'a>>>>,
    downstream: RefCell<DownstreamBuilder>,
    methods: RefCell<Option<Vec<String>>>,
}

impl<'a> RouteBuilder<'a> {
    pub fn build(&mut self) -> Route {
        let pattern = self.pattern.borrow().to_owned().unwrap();

        let routebuilders: &mut Option<Vec<RouteBuilder>> = self.children.get_mut();
        let routes = match routebuilders {
            Some(b) => {
                let mut result = vec![];
                for rb in b {
                    result.push(rb.build());
                }
                Some(result)
            }
            None => None,
        };
        let downstream = self.downstream.get_mut().build();

        //Build method hashset
        let mut methods: Option<HashSet<Method>> = None;
        {
            let mut methods_clone = (&self.methods).clone();
            let mb = methods_clone.get_mut();
            if mb.is_some() {
                let mut vec_methods: HashSet<Method> = HashSet::new();
                match mb {
                    Some(s) => {
                        for method_string in s {
                            let method =
                                Method::from_bytes(method_string.to_uppercase().as_bytes())
                                    .unwrap();
                            vec_methods.insert(method);
                        }
                        methods = Some(vec_methods);
                    }
                    None => {}
                }
            }
        }

        Route {
            pattern: Regex::new(&pattern).unwrap(),
            children: routes,
            downstream: downstream,
            methods: methods,
        }
    }
}
