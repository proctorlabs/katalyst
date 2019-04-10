#![allow(clippy::implicit_hasher)]
use super::*;
use crate::balancer::BalancerDirectory;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct HostsBuilder {
    #[serde(default = "default_balancer")]
    balancer: String,
    servers: Vec<String>,
}

fn default_balancer() -> String {
    "round_robin".to_string()
}

impl Builder<HashMap<String, Hosts>> for HashMap<String, HostsBuilder> {
    fn build(&self, engine: Arc<KatalystEngine>) -> Result<HashMap<String, Hosts>, KatalystError> {
        let balancers = engine.locate::<BalancerDirectory>()?;
        Ok(self
            .iter()
            .map(|v| {
                let builder = &balancers[&v.1.balancer.as_str()];
                (
                    v.0.clone(),
                    Hosts {
                        servers: builder.build(v.1.servers.clone()).unwrap(),
                    },
                )
            })
            .collect())
    }
}
