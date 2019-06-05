use crate::{app::Katalyst, context::*, modules::*};
use futures::future::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct WhitelistConfig {
    ips: Vec<String>,
}

#[derive(Default, Debug)]
pub struct WhitelistBuilder;

impl ModuleProvider for WhitelistBuilder {
    fn name(&self) -> &'static str {
        "whitelist"
    }

    fn build(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        config: &unstructured::Document,
    ) -> Result<Module> {
        let c: WhitelistConfig = config.clone().try_into().map_err(|_| {
            GatewayError::ConfigNotParseable("Host module configuration failed".into())
        })?;
        Ok(Whitelist { ips: c.ips }.into_module())
    }
}

#[derive(Default, Debug)]
pub struct Whitelist {
    ips: Vec<String>,
}

impl AuthenticatorModule for Whitelist {
    fn authenticate(&self, guard: RequestContext) -> ModuleResult {
        let metadata = ensure_fut!(guard.metadata());
        if self.ips.contains(&metadata.remote_ip) {
            Box::new(ok(()))
        } else {
            Box::new(err(GatewayError::Unauthorized))
        }
    }
}
