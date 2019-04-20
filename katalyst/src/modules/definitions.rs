use crate::app::KatalystEngine;
use crate::config::builder::*;
use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

pub enum ModuleType {
    Authenticator,
    RequestHandler,
}

pub enum ModuleConfig {
    RequestHandler(HandlerBuilder),
    Authenticator(AuthenticatorBuilder),
}

impl ModuleType {
    pub(crate) fn type_id(&self) -> &'static str {
        match self {
            ModuleType::Authenticator => "authenticator",
            ModuleType::RequestHandler => "request-handler",
        }
    }
}

pub type ModuleResult = Box<Future<Item = Context, Error = RequestFailure> + Send>;

pub trait ModuleDispatch: Send + Sync + Debug {
    fn dispatch(&self, ctx: Context) -> ModuleResult;
}

pub trait Module: Send + Sync + Debug {
    fn name(&self) -> &'static str;

    fn module_type(&self) -> ModuleType;

    fn build(
        &self,
        engine: Arc<KatalystEngine>,
        config: &ModuleConfig,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure>;
}

impl Module {
    pub(crate) fn key(&self) -> String {
        format!(
            "{id}-{mtype}",
            id = self.name(),
            mtype = self.module_type().type_id()
        )
    }
}
