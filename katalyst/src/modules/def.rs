use crate::prelude::*;
use std::{fmt::Debug, sync::Arc};
use unstructured::Document;

pub trait ModuleData {
    const MODULE_TYPE: ModuleType;
    type RUST_TYPE;
}

pub trait ModuleProvider: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &Document) -> Result<Module>;
}

macro_rules! impl_module {
    ($($name:ident, $trait:ident { $( $ret:ty: $method:ident => ( $($argname:ident : $argtype:ty),* ) )* })+) => {
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
                type RUST_TYPE = $name;
            }
        )*

        $(
            #[derive(Debug)]
            pub struct $name(pub Box<dyn $trait + Send>);

            pub trait $trait: Send + Sync + Debug {
                $(
                    fn $method(&self, $($argname: $argtype , )*) -> $ret;
                )*

                fn into_module(self) -> Module where Self: 'static + Sized {
                    Module::$name($name(Box::new(self)))
                }
            }

            impl From<Box<$trait + Send>> for $name {
                fn from(module: Box<$trait + Send>) -> Self {
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

pub type BalancerLease = Result<Arc<String>>;

impl_module! {
    Authenticator, AuthenticatorModule {
        AsyncResult<()>: authenticate => (guard: RequestContext)
    }

    Authorizer, AuthorizerModule {
        AsyncResult<()>: authorize => (guard: RequestContext)
    }

    CacheHandler, CacheHandlerModule {
        AsyncResult<()>: check_cache => (guard: RequestContext)
        AsyncResult<()>: update_cache => (guard: RequestContext)
    }

    CacheProvider, CacheProviderModule {
        AsyncResult<Arc<CachedObject>>: get_key => (key: &str)
        AsyncResult<()>: set_key => (key: &str, val: CachedObject)
    }

    Plugin, PluginModule {
        AsyncResult<()>: run => (guard: RequestContext)
    }

    RequestHandler, RequestHandlerModule {
        AsyncResult<()>: dispatch => (guard: RequestContext)
    }

    LoadBalancer, LoadBalancerModule {
        BalancerLease: lease => ()
    }
}
