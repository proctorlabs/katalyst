use crate::modules::RequestHook;
use http::Method;
use regex::Regex;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct Route {
    pub pattern: Regex,
    pub children: Option<Vec<Arc<Route>>>,
    pub handler: Arc<RequestHook>,
    pub plugins: Option<Vec<Arc<RequestHook>>>,
    pub authorizers: Option<Vec<Arc<RequestHook>>>,
    pub cache: Option<Arc<RequestHook>>,
    pub methods: Option<HashSet<Method>>,
    pub authenticators: Option<Vec<Arc<RequestHook>>>,
}
