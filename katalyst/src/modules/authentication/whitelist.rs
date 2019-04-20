use crate::app::KatalystEngine;
use crate::config::builder::AuthenticatorBuilder;
use crate::context::*;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;

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
        config: &ModuleConfig,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        match config {
            ModuleConfig::Authenticator(config) => match config {
                AuthenticatorBuilder::Whitelist { ips } => {
                    Ok(Arc::new(Whitelist { ips: ips.to_vec() }))
                }
                _ => Err(ConfigurationFailure::InvalidResource),
            },
            _ => Err(ConfigurationFailure::InvalidResource),
        }
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
