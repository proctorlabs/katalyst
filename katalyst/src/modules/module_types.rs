use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

macro_rules! impl_module {
    ($($name:ident, $trait:ident { $($method:tt)* });+) => {
        $(
            #[derive(Debug)]
            pub struct $name(pub Box<dyn $trait>);

            pub trait $trait: Send + Sync + Debug {
                $($method)*
            }
        )*
    };
}

impl_module! {
    Authorizer, AuthorizerModule {
        fn run(&self, ctx: Context) -> ModuleResult;
    }
}

#[derive(Debug)]
pub struct Authenticator(pub Box<dyn RequestHook>);

#[derive(Debug)]
pub struct CacheProvider(pub Box<dyn CacheProviderModule>);

#[derive(Debug)]
pub struct CacheHandler(pub Box<dyn RequestHook>);

#[derive(Debug)]
pub struct RequestHandler(pub Box<dyn RequestHook>);

#[derive(Debug)]
pub struct Plugin(pub Box<dyn RequestHook>);

pub trait CacheProviderModule: Send + Sync + Debug {
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

macro_rules! delegate_requesthook {
    ($($module:ident);+) => {
        $(
            impl RequestHook for $module {
                #[inline]
                fn run(&self, ctx: Context) -> ModuleResult {
                    self.0.run(ctx)
                }
            }
        )*
    };
}

delegate_requesthook! {
    Authenticator;
    CacheHandler;
    RequestHandler;
    Plugin
}

impl CacheProviderModule for CacheProvider {
    #[inline]
    fn get_key(&self, key: &str) -> Box<Future<Item = Arc<Vec<u8>>, Error = GatewayError> + Send> {
        self.0.get_key(key)
    }

    #[inline]
    fn set_key(
        &self,
        key: &str,
        val: Vec<u8>,
    ) -> Box<Future<Item = (), Error = GatewayError> + Send> {
        self.0.set_key(key, val)
    }
}

macro_rules! impl_module_variants {
    ($($variant:ident, $name:ty);+) => {
        #[derive(PartialEq, Debug)]
        pub enum ModuleType {
            $($variant,)*
        }

        #[derive(Debug)]
        pub enum Module {
            $($variant($name),)*
        }

        impl Module {
            pub fn get_type(&self) -> ModuleType {
                match self {
                    $(
                        Module::$variant(_) => ModuleType::$variant,
                    )*
                }
            }

            pub fn is_type(&self, t: ModuleType) -> bool {
                t == self.get_type()
            }
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

impl_module_variants! {
    Authenticator, Authenticator;
    Authorizer, Authorizer;
    CacheProvider, CacheProvider;
    CacheHandler, CacheHandler;
    RequestHandler, RequestHandler;
    Plugin, Plugin
}
