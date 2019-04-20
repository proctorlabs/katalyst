use crate::app::KatalystEngine;
use crate::context::*;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct WhitelistConfig {
    ips: Vec<String>,
}

#[derive(Default, Debug)]
pub struct WhitelistBuilder {}

impl Module for WhitelistBuilder {
    fn name(&self) -> &'static str {
        "whitelist"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::Authenticator
    }

    fn build(
        &self,
        _: Arc<KatalystEngine>,
        config: &ModuleConfigLoader,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        let c: WhitelistConfig = config.load()?;
        Ok(Arc::new(Whitelist { ips: c.ips }))
    }
}

#[derive(Default, Debug)]
pub struct Whitelist {
    ips: Vec<String>,
}

impl ModuleDispatch for Whitelist {
    fn dispatch(&self, ctx: Context) -> ModuleResult {
        if self.ips.contains(&ctx.detail.remote_ip) {
            Box::new(ok::<Context, RequestFailure>(ctx))
        } else {
            Box::new(err::<Context, RequestFailure>(RequestFailure::Unauthorized))
        }
    }
}
