use super::*;
use crate::instance::Route;
use std::collections::HashMap;
use std::sync::Arc;

/// Detail on the matched route currently associated with this context
#[derive(Debug)]
pub enum Match {
    /// No route has been matched yet
    Unmatched,
    /// A route has been matched
    Matched {
        /// The detail of the matched route
        route: Arc<Route>,
        /// Hashmap of variables that have been captured from this route match
        captures: HashMap<String, String>,
    },
}

impl Match {
    /// Returns a boolean indicating if a route has been matched to this request
    pub fn is_matched(&self) -> bool {
        match self {
            Match::Matched { .. } => true,
            Match::Unmatched => false,
        }
    }

    /// Return the details of the route attached to this request. Will return a
    /// GatewayError::RequestFailed with a status code of NOT_FOUND if the route
    /// has not been matched.
    pub fn route(&self) -> Result<Arc<Route>> {
        match self {
            Match::Matched { route, .. } => Ok(route.clone()),
            _ => fail!(NOT_FOUND),
        }
    }

    /// Retrieve the captures value for the specified key for the current route.
    /// Will return a GatewayError::RequestFailed with a status code of NOT_FOUND
    /// if the route has not been matched or if the capture does not exist.
    pub fn get_value(&self, key: &str) -> Result<String> {
        match self {
            Match::Matched { captures, .. } => {
                Ok(captures.get(key).ok_or_else(|| fail!(_ NOT_FOUND))?.to_string())
            }
            _ => fail!(NOT_FOUND),
        }
    }
}
