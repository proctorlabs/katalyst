use crate::KatalystTemplatePlaceholder;
use http::Method;
use std::collections::HashSet;

pub mod builder;
pub mod parsers;

use regex::Regex;

#[derive(Clone)]
pub struct Gateway {
    pub routes: Vec<Route>,
    pub listener: Listener,
}

#[derive(Clone)]
pub struct Route {
    pub pattern: Regex,
    pub children: Option<Vec<Route>>,
    pub downstream: Downstream,
    pub methods: Option<HashSet<Method>>,
}

#[derive(Clone)]
pub struct Listener {
    pub interface: String,
}

pub struct Downstream {
    pub base_url: String,
    pub path_parts: Vec<Box<KatalystTemplatePlaceholder>>,
}

impl Clone for Downstream {
    fn clone(&self) -> Self {
        let mut cloned_placeholders = vec![];
        for ph in self.path_parts.iter() {
            cloned_placeholders.push(ph.duplicate());
        }
        Downstream {
            base_url: self.base_url.to_string(),
            path_parts: vec![],
        }
    }
}
