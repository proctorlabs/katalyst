use crate::app::Katalyst;
use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;
use unstructured::Document;

pub type ModuleResultSync = std::result::Result<Context, ModuleError>;
pub type ModuleResult = Box<Future<Item = Context, Error = ModuleError> + Send>;

#[derive(PartialEq, Debug)]
pub enum ModuleType {
    Authenticator,
    Authorizer,
    RequestHandler,
    Plugin,
    CacheProvider,
    CacheHandler,
}

pub trait ModuleProvider: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &Document) -> Result<Module>;
}

pub trait CacheProvider: Send + Sync + Debug {
    fn get_key(&self, key: &str) -> Box<Future<Item = Arc<Vec<u8>>, Error = GatewayError> + Send>;

    fn set_key(
        &self,
        key: &str,
        val: Vec<u8>,
    ) -> Box<Future<Item = (), Error = GatewayError> + Send>;
}

pub trait ModuleDispatch: Send + Sync + Debug {
    fn dispatch(&self, ctx: Context) -> ModuleResult;
}

pub enum Module {
    Authenticator(Arc<ModuleDispatch>),
    Authorizer(Arc<ModuleDispatch>),
    RequestHandler(Arc<ModuleDispatch>),
    Plugin(Arc<ModuleDispatch>),
    CacheProvider(Arc<CacheProvider>),
    CacheHandler(Arc<ModuleDispatch>),
}
