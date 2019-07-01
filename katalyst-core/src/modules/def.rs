use crate::prelude::*;
use std::{fmt::Debug, sync::Arc};
use unstructured::Document;

#[doc(hidden)]
pub trait ModuleData {
    const MODULE_TYPE: ModuleType;
    type RUST_TYPE;
}

/// Required trait for any modules to be registered
pub trait ModuleProvider: Send + Sync + Debug {
    /// The name of the module, matched to the "type" field in configuration
    fn name(&self) -> &'static str;
    /// The method used to build a module.
    fn build(&self, _: ModuleType, _: &Document) -> Result<Module>;
}

macro_rules! impl_module {
    ($($sname:expr, $name:ident, $trait:ident { $( $ret:ty: $method:ident => ( $($argname:ident : $argtype:ty),* ) )* })+) => {
        /// Variants corresponding to each module type.
        #[derive(PartialEq, Debug)]
        pub enum ModuleType {
            $(
                #[doc = $sname]
                #[doc = " module type."]
                $name,
            )*
        }

        /// This enum is a container for all module types.
        #[derive(Debug)]
        pub enum Module {
            $(
                #[doc = "Variant containing "]
                #[doc = $sname]
                #[doc = " modules."]
                $name($name),
            )*
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
            #[doc = $sname]
            #[doc = " container."]
            #[derive(Debug)]
            pub struct $name(pub Box<dyn $trait + Send>);

            #[doc = "Implement this trait when building "]
            #[doc = $sname]
            #[doc = " modules."]
            pub trait $trait: Send + Sync + Debug {
                $(
                    #[doc = "Method implementation for "]
                    #[doc = $sname]
                    #[doc = " modules."]
                    fn $method(&self, $($argname: $argtype , )*) -> $ret;
                )*

                /// Box this module into the Module enum.
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

/// A lease from a load balancer
pub type BalancerLease = Result<Arc<String>>;

impl_module! {
    "Authenticator", Authenticator, AuthenticatorModule {
        AsyncResult<()>: authenticate => (guard: RequestContext)
    }

    "Authorizer", Authorizer, AuthorizerModule {
        AsyncResult<()>: authorize => (guard: RequestContext)
    }

    "CacheHandler", CacheHandler, CacheHandlerModule {
        AsyncResult<()>: check_cache => (guard: RequestContext)
        AsyncResult<()>: update_cache => (guard: RequestContext)
    }

    "CacheProvider", CacheProvider, CacheProviderModule {
        AsyncResult<Arc<CachedObject>>: get_key => (key: &str)
        AsyncResult<()>: set_key => (key: &str, val: CachedObject)
    }

    "Plugin", Plugin, PluginModule {
        AsyncResult<()>: run => (guard: RequestContext)
    }

    "RequestHandler", RequestHandler, RequestHandlerModule {
        AsyncResult<()>: dispatch => (guard: RequestContext)
    }

    "LoadBalancer", LoadBalancer, LoadBalancerModule {
        BalancerLease: lease => ()
    }
}
//
