use crate::app::KatalystEngine;
use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

pub enum ModuleType {
    Authenticator,
    Authorizer,
    RequestHandler,
    Plugin,
}

impl ModuleType {
    pub fn type_id(&self) -> &'static str {
        match self {
            ModuleType::Authenticator => AuthenticatorModule::KEY,
            ModuleType::RequestHandler => HandlerModule::KEY,
            ModuleType::Plugin => PluginModule::KEY,
            ModuleType::Authorizer => AuthorizerModule::KEY,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct AuthenticatorModule {}
impl TypeId for AuthenticatorModule {
    const KEY: &'static str = "authenticator";
}

#[derive(Default, Clone, Debug)]
pub struct AuthorizerModule {}
impl TypeId for AuthorizerModule {
    const KEY: &'static str = "authorizer";
}

#[derive(Default, Clone, Debug)]
pub struct HandlerModule {}
impl TypeId for HandlerModule {
    const KEY: &'static str = "request-handler";
}

#[derive(Default, Clone, Debug)]
pub struct PluginModule {}
impl TypeId for PluginModule {
    const KEY: &'static str = "plugin";
}

pub trait TypeId {
    const KEY: &'static str;
}

pub struct ModuleError {
    pub error: RequestFailure,
    pub context: Context,
}

impl Context {
    pub fn fail(self, error: RequestFailure) -> ModuleError {
        ModuleError {
            error,
            context: self,
        }
    }
}

pub type ModuleResultSync = Result<Context, ModuleError>;
pub type ModuleResult = Box<Future<Item = Context, Error = ModuleError> + Send>;

pub trait ModuleDispatch: Send + Sync + Debug {
    fn dispatch(&self, ctx: Context) -> ModuleResult;
}

pub struct ModuleConfigLoader {
    pub(crate) raw: serde_value::Value,
}

impl ModuleConfigLoader {
    pub fn load<T>(&self) -> Result<T, ConfigurationFailure>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let c: T = self.raw.clone().deserialize_into().map_err(|e| {
            ConfigurationFailure::ConfigNotParseable(format!(
                "Could not parse module configuration: {}",
                e
            ))
        })?;
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
