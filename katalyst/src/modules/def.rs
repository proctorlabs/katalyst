use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;
use unstructured::Document;

pub type ModuleResultSync = std::result::Result<Context, ModuleError>;
pub type ModuleResult = Box<Future<Item = Context, Error = ModuleError> + Send>;

pub trait ModuleData {
    const MODULE_TYPE: ModuleType;
}

pub trait ModuleProvider: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &Document) -> Result<Module>;
}

macro_rules! impl_module {
    ($($name:ident, $trait:ident { $( $ret:ty: $method:ident => $($argname:ident : $argtype:ty),*);* });+) => {
        #[derive(PartialEq, Debug)]
        pub enum ModuleType {
            $($name,)*
        }

        #[derive(Debug)]
        pub enum Module {
            $($name($name),)*
        }

        impl Module {
            pub fn get_type(&self) -> ModuleType {
                match self {
                    $(
                        Module::$name(_) => ModuleType::$name,
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
                    Module::$name(module)
                }
            }

            impl ModuleData for $name {
                const MODULE_TYPE: ModuleType = ModuleType::$name;
            }
        )*

        $(
            #[derive(Debug)]
            pub struct $name(pub Box<dyn $trait>);

            pub trait $trait: Send + Sync + Debug {
                $(
                    fn $method(&self, $($argname: $argtype , )*) -> $ret;
                )*

                fn into_module(self) -> Module where Self: 'static + Sized {
                    Module::$name($name(Box::new(self)))
                }
            }

            impl From<Box<$trait>> for $name {
                fn from(module: Box<$trait>) -> Self {
                    $name(module)
                }
            }

            impl $trait for $name {
                $(
                    #[inline]
                    fn $method(&self, $($argname: $argtype , )*) -> $ret {
                        self. 0 .$method($($argname,)*)
                    }
                )*
            }
        )*
    };
}

impl_module! {
    Authenticator, AuthenticatorModule {
        ModuleResult: authenticate => ctx: Context
    };

    Authorizer, AuthorizerModule {
        ModuleResult: authorize => ctx: Context
    };

    CacheHandler, CacheHandlerModule {
        ModuleResult: check_cache => ctx: Context
    };

    CacheProvider, CacheProviderModule {
        Box<Future<Item = Arc<Vec<u8>>, Error = GatewayError> + Send>: get_key => key: &str;
        Box<Future<Item = (), Error = GatewayError> + Send>: set_key => key: &str, val: Vec<u8>
    };

    Plugin, PluginModule {
        ModuleResult: run => ctx: Context
    };

    RequestHandler, RequestHandlerModule {
        ModuleResult: dispatch => ctx: Context
    }
}
