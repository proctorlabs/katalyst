use crate::config::Route;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::string::String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteBuilder<'a> {
    pub pattern: RefCell<Option<String>>,
    pub children: RefCell<Option<Vec<RouteBuilder<'a>>>>,
}

impl<'a> RouteBuilder<'a> {
    pub fn build(&mut self) -> Route {
        let pattern = self.pattern.borrow().to_owned().unwrap();
        Route {
            pattern: Regex::new(&pattern).unwrap(),
            children: None,
        }
    }
}
