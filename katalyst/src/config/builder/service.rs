use super::*;
use crate::{
    app::Katalyst,
    instance::{Interface, Service},
    modules::*,
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
                addr: self.address.parse().map_err(|_| {
                    GatewayError::InvalidAddress("Service listener address is invalid")
                })?,
                cert: self.ssl_cert.clone(),
                key: self.ssl_key.clone(),
            }
        } else {
            Interface::Http {
                addr: self
                    .address
                    .parse()
                    .map_err(|_| InvalidAddress("Service listener address is invalid"))?,
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

macro_rules! module {
    ($name:ident, $mt:expr) => {
        Arc::new(match $mt {
            Module::$name(mtch) => mtch,
            _ => return Err(GatewayError::FeatureUnavailable),
        })
    };
}

impl Builder<Service> for ServiceBuilder {
    fn build(&self, instance: Arc<Katalyst>) -> Result<Service> {
        Ok(Service {
            interfaces: self
                .interfaces
                .iter()
                .map(|i| i.make_interface())
                .collect::<Result<Vec<Interface>>>()?,
            cache: module!(CacheProvider, self.cache.build(instance.clone())?),
        })
    }
}
