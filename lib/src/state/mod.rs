use crate::KatalystTemplatePlaceholder;
use http::Method;
use regex::Regex;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct KatalystState {
    pub routes: Vec<Arc<Route>>,
    pub listener: Listener,
}

#[derive(Debug)]
pub struct Route {
    pub pattern: Regex,
    pub children: Option<Vec<Arc<Route>>>,
    pub downstream: Downstream,
    pub methods: Option<HashSet<Method>>,
}

#[derive(Debug)]
pub struct Listener {
    pub interface: String,
}

impl Default for Listener {
    fn default() -> Self {
        Listener {
            interface: "0.0.0.0:8080".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Downstream {
    pub base_url: String,
    pub path_parts: Vec<Box<KatalystTemplatePlaceholder>>,
}
