use crate::config::Downstream;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::string::String;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DownstreamBuilder {
    base_url: RefCell<String>,
    path: RefCell<String>,
}

impl<'a> DownstreamBuilder {
    pub fn build(&mut self) -> Downstream {
        Downstream {
            base_url: self.base_url.borrow().to_owned(),
            path: self.path.borrow().to_owned(),
        }
    }
}
