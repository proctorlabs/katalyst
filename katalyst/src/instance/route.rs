use crate::modules::ModuleDispatch;
use http::Method;
use regex::Regex;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct Route {
    pub pattern: Regex,
    pub children: Option<Vec<Arc<Route>>>,
    pub handler: Arc<ModuleDispatch>,
    pub plugins: Option<Vec<Arc<ModuleDispatch>>>,
    pub authorizers: Option<Vec<Arc<ModuleDispatch>>>,
    pub cache: Option<Arc<ModuleDispatch>>,
    pub methods: Option<HashSet<Method>>,
    pub authenticators: Option<Vec<Arc<ModuleDispatch>>>,
}
