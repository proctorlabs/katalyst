use crate::authentication::KatalystAuthenticator;
use crate::balancer::{default_balancer, KatalystBalancer};
use crate::templates::KatalystTemplatePlaceholder;
use http::Method;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct KatalystState {
    pub hosts: HashMap<String, Hosts>,
    pub routes: Vec<Arc<Route>>,
    pub listener: Listener,
}

#[derive(Debug)]
pub struct Hosts {
    pub servers: Arc<KatalystBalancer>,
}

impl Default for Hosts {
    fn default() -> Self {
        Hosts {
            servers: default_balancer(),
        }
    }
}

#[derive(Debug)]
pub struct Route {
    pub pattern: Regex,
    pub children: Option<Vec<Arc<Route>>>,
    pub downstream: Downstream,
    pub methods: Option<HashSet<Method>>,
    pub authenticators: Option<Vec<Authenticator>>,
}

#[derive(Debug)]
pub struct Authenticator {
    pub authenticator: Arc<KatalystAuthenticator>,
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
    pub host: String,
    pub path_parts: Vec<Box<KatalystTemplatePlaceholder>>,
}
