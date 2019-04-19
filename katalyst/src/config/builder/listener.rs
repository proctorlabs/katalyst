use super::*;
use crate::app::KatalystEngine;
use crate::instance::Listener;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListenerBuilder {
    pub interface: String,
}

impl Builder<Listener> for ListenerBuilder {
    fn build(&self, _: Arc<KatalystEngine>) -> Result<Listener, ConfigurationFailure> {
        Ok(Listener {
            interface: self.interface.to_owned(),
        })
    }
}
