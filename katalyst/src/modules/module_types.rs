use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AuthorizerModule(pub Arc<dyn RequestHook>);

#[derive(Clone, Debug)]
pub struct AuthenticatorModule(pub Arc<dyn RequestHook>);

#[derive(Clone, Debug)]
pub struct CacheProviderModule(pub Arc<dyn CacheProvider>);

#[derive(Clone, Debug)]
pub struct CacheHandlerModule(pub Arc<dyn RequestHook>);

#[derive(Clone, Debug)]
pub struct RequestHandlerModule(pub Arc<dyn RequestHook>);

#[derive(Clone, Debug)]
pub struct PluginModule(pub Arc<dyn RequestHook>);

macro_rules! impl_module {
    ($($variant:ident,$name:ident);*) => {

        #[derive(PartialEq, Debug)]
        pub enum ModuleType {
            $($variant,)*
        }

        pub enum Module {
            $($variant($name),)*
        }

        $(
        impl From<$name> for Module {
            fn from(module: $name) -> Self {
                Module::$variant(module)
            }
        }

        impl ModuleData for $name {
            const MODULE_TYPE: ModuleType = ModuleType::$variant;
        }
        )*
    };
}

impl_module! {
    Authenticator, AuthenticatorModule;
    Authorizer, AuthorizerModule;
    CacheProvider, CacheProviderModule;
    CacheHandler, CacheHandlerModule;
    RequestHandler, RequestHandlerModule;
    Plugin, PluginModule
}

pub trait CacheProvider: Send + Sync + Debug {
    fn get_key(&self, key: &str) -> Box<Future<Item = Arc<Vec<u8>>, Error = GatewayError> + Send>;

    fn set_key(
        &self,
        key: &str,
        val: Vec<u8>,
    ) -> Box<Future<Item = (), Error = GatewayError> + Send>;
}

pub trait RequestHook: Send + Sync + Debug {
    fn run(&self, ctx: Context) -> ModuleResult;
}
