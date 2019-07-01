use super::*;
use crate::{
    instance::{Interface, Service},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Builder for an interface attached to this service
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InterfaceBuilder {
    address: String,
    ssl: bool,
    ssl_cert: String,
    ssl_key: String,
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

/// Builder for a KatalystCore service instance
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ServiceBuilder {
    interfaces: Vec<InterfaceBuilder>,
    cache: ModuleBuilder<CacheProvider>,
}

impl Builder<Service> for ServiceBuilder {
    fn build(&self) -> Result<Service> {
        Ok(Service {
            interfaces: self
                .interfaces
                .iter()
                .map(|i| i.make_interface())
                .collect::<Result<Vec<Interface>>>()?,
            cache: module_unwrap!(CacheProvider, self.cache.build()?),
        })
    }
}
