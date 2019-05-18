use crate::app::Katalyst;
use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(PartialEq, Debug)]
pub enum ModuleType {
    Authenticator,
    Authorizer,
    RequestHandler,
    Plugin,
    CacheProvider,
    CacheHandler,
}

pub type ModuleResultSync = Result<Context, ModuleError>;
pub type ModuleResult = Box<Future<Item = Context, Error = ModuleError> + Send>;

pub trait Module: Send + Sync + Debug {
    fn name(&self) -> &'static str;

    fn supported_hooks(&self) -> Vec<ModuleType>;

    fn build_hook(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        _: &unstructured::Document,
    ) -> Result<Arc<ModuleDispatch>, GatewayError> {
        Err(GatewayError::InvalidResource)
    }

    fn build_cache(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        _: &unstructured::Document,
    ) -> Result<Arc<CacheProvider>, GatewayError> {
        Err(GatewayError::InvalidResource)
    }
}
