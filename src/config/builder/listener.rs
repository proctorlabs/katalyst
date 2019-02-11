use crate::config::Listener;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListenerBuilder {
    interface: RefCell<Option<String>>,
}

impl<'a> ListenerBuilder {
    pub fn build(&mut self) -> Listener {
        Listener {
            interface: self.interface.borrow().to_owned().unwrap(),
        }
    }
}
