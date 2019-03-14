use super::*;
use serde::{Deserialize, Serialize};
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct HostsBuilder {
    name: String,
    servers: Vec<String>,
}

impl Builder<Hosts> for HostsBuilder {
    fn build(&self, _: Arc<KatalystEngine>) -> Result<Hosts, KatalystError> {
        Ok(Hosts {
            name: self.name.to_string(),
            servers: self.servers.clone(),
        })
    }
}

impl Builder<Vec<Hosts>> for Vec<HostsBuilder> {
    fn build(&self, engine: Arc<KatalystEngine>) -> Result<Vec<Hosts>, KatalystError> {
        Ok(self
            .iter()
            .map(|h| h.build(engine.clone()).unwrap())
            .collect())
    }
}
