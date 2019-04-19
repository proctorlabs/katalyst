use super::*;
use http::Method;
use regex::Regex;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct Route {
    pub pattern: Regex,
    pub children: Option<Vec<Arc<Route>>>,
    pub handler: Handler,
    pub methods: Option<HashSet<Method>>,
    pub authenticators: Option<Vec<Authenticator>>,
}
