use super::*;
use crate::app::Katalyst;
use crate::instance::Service;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::modules::*;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ServiceBuilder {
    pub interface: String,
    pub cache: ModuleBuilder<CacheModule>,
}

impl Builder<Service> for ServiceBuilder {
    fn build(&self, instance: Arc<Katalyst>) -> Result<Service, ConfigurationFailure> {
        Ok(Service {
            interface: self.interface.parse().map_err(|_| {
                ConfigurationFailure::InvalidAddress("Service listener address is invalid")
            })?,
            cache: self.cache.build(instance.clone())?,
        })
    }
}
