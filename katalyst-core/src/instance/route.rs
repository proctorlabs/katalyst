use crate::modules::*;
use http::Method;
use regex::Regex;
use std::{collections::HashSet, sync::Arc};

/// Modules and other data associated with a specific route
#[derive(Debug)]
pub struct Route {
    /// The URI pattern used to match a request to this route
    pub pattern: Regex,
    /// Child routes
    pub children: Option<Vec<Arc<Route>>>,
    /// The request handler for this route
    pub handler: Arc<RequestHandler>,
    /// Plugin modules for this route
    pub plugins: Option<Vec<Arc<Plugin>>>,
    /// Authorization modules for this route
    pub authorizers: Option<Vec<Arc<Authorizer>>>,
    /// Cache handler for this route
    pub cache: Option<Arc<CacheHandler>>,
    /// Valid methods for this route. If `None` then any method is allowed
    pub methods: Option<HashSet<Method>>,
    /// Authenticator modules for this route
    pub authenticators: Option<Vec<Arc<Authenticator>>>,
}
