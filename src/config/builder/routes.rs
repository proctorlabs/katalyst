use crate::config::Route;
use regex::Regex;
use std::cell::RefCell;
use std::string::String;

#[derive(Clone, Debug)]
pub struct RouteBuilder<'a> {
    pub pattern: RefCell<Option<String>>,
    pub children: RefCell<Option<Vec<RouteBuilder<'a>>>>,
}

impl<'a> RouteBuilder<'a> {
    pub fn set_pattern(self, new_pattern: String) -> Self {
        self.pattern.replace(Some(new_pattern.to_owned()));
        self
    }

    pub fn add_children(self, new_routes: &mut Vec<RouteBuilder<'a>>) -> Self {
        let mut children: Vec<RouteBuilder> = match self.children.borrow().to_owned() {
            Some(s) => s,
            None => vec![],
        };
        children.append(new_routes);
        self.children.replace(Some(children));
        self
    }

    pub fn build(&mut self) -> Route {
        let pattern = self.pattern.borrow().to_owned().unwrap();
        Route {
            pattern: Regex::new(&pattern).unwrap(), //Regex::new(&(self.pattern.borrow().unwrap())).unwrap().clone(),
            children: None,
        }
    }

    pub fn new() -> Self {
        let result: RouteBuilder<'a> = RouteBuilder {
            pattern: RefCell::new(None),
            children: RefCell::new(None),
        };
        result
    }
}
