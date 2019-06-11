use super::*;
use crate::{
    app::Katalyst,
    instance::{Interface, Service},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InterfaceBuilder {
    pub address: String,
    pub ssl: bool,
    pub ssl_cert: String,
    pub ssl_key: String,
}

impl InterfaceBuilder {
    fn make_interface(&self) -> Result<Interface> {
        Ok(if self.ssl {
            Interface::Https {
                addr: self.address.parse().map_err(|e| {
                    err!(
                        ConfigurationFailure,
                        format!("Failed to parse the listener address {}", self.address),
                        e
                    )
                })?,
                cert: self.ssl_cert.clone(),
                key: self.ssl_key.clone(),
            }
        } else {
            Interface::Http {
                addr: self.address.parse().map_err(|e| {
                    err!(
                        ConfigurationFailure,
                        format!("Failed to parse the listener address {}", self.address),
                        e
                    )
                })?,
            }
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ServiceBuilder {
    pub interfaces: Vec<InterfaceBuilder>,
    pub cache: ModuleBuilder<CacheProvider>,
}

impl Builder<Service> for ServiceBuilder {
    fn build(&self, instance: Arc<Katalyst>) -> Result<Service> {
        Ok(Service {
            interfaces: self
                .interfaces
                .iter()
                .map(|i| i.make_interface())
                .collect::<Result<Vec<Interface>>>()?,
            cache: module_unwrap!(CacheProvider, self.cache.build(instance.clone())?),
        })
    }
}
