use super::*;
use crate::app::Katalyst;
use crate::instance::Service;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ServiceBuilder {
    pub interface: String,
}

impl Builder<Service> for ServiceBuilder {
    fn build(&self, _: Arc<Katalyst>) -> Result<Service, ConfigurationFailure> {
        Ok(Service {
            interface: self.interface.parse().map_err(|_| {
                ConfigurationFailure::InvalidAddress("Service listener address is invalid")
            })?,
        })
    }
}
