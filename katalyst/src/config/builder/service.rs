use super::*;
use crate::app::Katalyst;
use crate::instance::Service;
use crate::modules::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ServiceBuilder {
    pub interface: String,
    pub cache: ModuleBuilder<CacheProviderModule>,
}

macro_rules! module {
    ($name:ident, $mt:expr) => {
        match $mt {
            Module::$name(mtch) => mtch.0,
            _ => return Err(GatewayError::FeatureUnavailable),
        }
    };
}

impl Builder<Service> for ServiceBuilder {
    fn build(&self, instance: Arc<Katalyst>) -> Result<Service, GatewayError> {
        Ok(Service {
            interface: self
                .interface
                .parse()
                .map_err(|_| GatewayError::InvalidAddress("Service listener address is invalid"))?,
            cache: module!(CacheProvider, self.cache.build(instance.clone())?),
        })
    }
}
