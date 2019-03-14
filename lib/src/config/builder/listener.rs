use super::*;
use crate::app::KatalystEngine;
use crate::error::KatalystError;
use crate::state::Listener;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListenerBuilder {
    pub interface: String,
}

impl Builder<Listener> for ListenerBuilder {
    fn build(&self, _: Arc<KatalystEngine>) -> Result<Listener, KatalystError> {
        Ok(Listener {
            interface: self.interface.to_owned(),
        })
    }
}
