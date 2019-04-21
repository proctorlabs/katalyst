use crate::app::KatalystEngine;
use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

pub enum ModuleType {
    Authenticator,
    RequestHandler,
}

impl ModuleType {
    pub fn type_id(&self) -> &'static str {
        match self {
            ModuleType::Authenticator => AuthenticatorModule::KEY,
            ModuleType::RequestHandler => HandlerModule::KEY,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct AuthenticatorModule {}
impl TypeId for AuthenticatorModule {
    const KEY: &'static str = "authenticator";
}

#[derive(Default, Clone, Debug)]
pub struct HandlerModule {}
impl TypeId for HandlerModule {
    const KEY: &'static str = "request-handler";
}

pub trait TypeId {
    const KEY: &'static str;
}

pub type ModuleResult = Box<Future<Item = Context, Error = RequestFailure> + Send>;

pub trait ModuleDispatch: Send + Sync + Debug {
    fn dispatch(&self, ctx: Context) -> ModuleResult;
}

pub struct ModuleConfigLoader {
    pub(crate) raw: serde_json::Value,
}

impl ModuleConfigLoader {
    pub fn load<T>(&self) -> Result<T, ConfigurationFailure>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let c: T = serde_json::from_value(self.raw.clone())?;
        Ok(c)
    }
}

pub trait Module: Send + Sync + Debug {
    fn name(&self) -> &'static str;

    fn module_type(&self) -> ModuleType;

    fn build(
        &self,
        engine: Arc<KatalystEngine>,
        config: &ModuleConfigLoader,
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