use super::*;
use crate::instance::Route;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub enum Match {
    Unmatched,
    Matched { route: Arc<Route>, captures: HashMap<String, String> },
}

impl Match {
    pub fn is_matched(&self) -> bool {
        match self {
            Match::Matched { .. } => true,
            Match::Unmatched => false,
        }
    }

    pub fn route(&self) -> Result<Arc<Route>> {
        match self {
            Match::Matched { route, .. } => Ok(route.clone()),
            _ => fail!(NOT_FOUND),
        }
    }

    pub fn get_value(&self, key: &str) -> Result<String> {
        match self {
            Match::Matched { captures, .. } => {
                Ok(captures.get(key).ok_or_else(|| fail!(_ NOT_FOUND))?.to_string())
            }
            _ => fail!(NOT_FOUND),
        }
    }
}
