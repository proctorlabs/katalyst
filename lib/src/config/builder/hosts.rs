use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct HostsBuilder {
    servers: Vec<String>,
}

impl Builder<HashMap<String, Hosts>> for HashMap<String, HostsBuilder> {
    fn build(&self, _: Arc<KatalystEngine>) -> Result<HashMap<String, Hosts>, KatalystError> {
        Ok(self
            .iter()
            .map(|v| {
                (
                    v.0.clone(),
                    Hosts {
                        servers: v.1.servers.clone(),
                    },
                )
            })
            .collect())
    }
}
