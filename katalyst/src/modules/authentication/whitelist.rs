use crate::app::Katalyst;
use crate::context::*;
use crate::modules::*;
use futures::future::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct WhitelistConfig {
    ips: Vec<String>,
}

#[derive(Default, Debug)]
pub struct WhitelistBuilder;

impl Module for WhitelistBuilder {
    fn name(&self) -> &'static str {
        "whitelist"
    }

    fn supported_hooks(&self) -> Vec<ModuleType> {
        vec![ModuleType::Authenticator]
    }

    fn build_hook(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        config: &unstructured::Document,
    ) -> Result<Arc<ModuleDispatch>> {
        let c: WhitelistConfig = config.clone().try_into().map_err(|_| {
            GatewayError::ConfigNotParseable("Host module configuration failed".into())
        })?;
        Ok(Arc::new(Whitelist { ips: c.ips }))
    }
}

#[derive(Default, Debug)]
pub struct Whitelist {
    ips: Vec<String>,
}

impl ModuleDispatch for Whitelist {
    fn dispatch(&self, ctx: Context) -> ModuleResult {
        if self.ips.contains(&ctx.metadata.remote_ip) {
            Box::new(ok(ctx))
        } else {
            Box::new(err(ctx.fail(GatewayError::Unauthorized)))
        }
    }
}
