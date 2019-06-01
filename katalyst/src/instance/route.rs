use crate::modules::*;
use http::Method;
use regex::Regex;
use std::{collections::HashSet, sync::Arc};

#[derive(Debug)]
pub struct Route {
    pub pattern: Regex,
    pub children: Option<Vec<Arc<Route>>>,
    pub handler: Arc<RequestHandler>,
    pub plugins: Option<Vec<Arc<Plugin>>>,
    pub authorizers: Option<Vec<Arc<Authorizer>>>,
    pub cache: Option<Arc<CacheHandler>>,
    pub methods: Option<HashSet<Method>>,
    pub authenticators: Option<Vec<Arc<Authenticator>>>,
}
