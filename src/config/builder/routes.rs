use crate::config::Route;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::string::String;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RouteBuilder<'a> {
    pattern: RefCell<Option<String>>,
    children: RefCell<Option<Vec<RouteBuilder<'a>>>>,
    message: RefCell<Option<String>>,
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

        Route {
            pattern: Regex::new(&pattern).unwrap(),
            children: routes,
            message: self.message.borrow().to_owned(),
        }
    }
}
