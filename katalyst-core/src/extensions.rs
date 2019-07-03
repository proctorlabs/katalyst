/*!
This module provides helpers for extending Katalyst functionality. Look at the
bind_katalyst! macro for a simple way to add extensions.
*/
use crate::{expression::ExpressionBinding, modules::ModuleProvider, prelude::*};
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    static ref MODULES: Mutex<HashMap<String, Arc<dyn ModuleProvider>>> = {
        let res = HashMap::new();
        Mutex::new(res)
    };
    static ref EXPRESSIONS: Mutex<HashMap<String, Arc<dyn ExpressionBinding>>> = {
        let res = HashMap::new();
        Mutex::new(res)
    };
}

/// This macro adds the loader entrypoint for this crate.
#[macro_export]
macro_rules! katalyst_link {
    (modules: { $( $module:expr ),* } ) => {
        #[doc(hidden)]
        pub fn __katalyst_init_modules() {
            $( $crate::extensions::bind_module(std::sync::Arc::new($module)); )*
        }
    };
    (expressions: { $( $expression:expr ),* } ) => {
        #[doc(hidden)]
        pub fn __katalyst_init_expressions() {
            $( $crate::extensions::bind_expression(std::sync::Arc::new($expression)); )*
        }
    };
}

/// Load modules or extensions from these crates
#[macro_export]
macro_rules! katalyst_load {
    (modules : { $( $krate:ident ),* } ) => {
        $( $krate ::__katalyst_init_modules(); )*
    };
    (expressions : { $( $krate:ident ),* } ) => {
        $( $krate ::__katalyst_init_expressions(); )*
    };
}

/// Retrieve a loaded module provider
pub fn get_module(name: &str) -> Result<Arc<dyn ModuleProvider>> {
    let reader = MODULES.lock();
    match reader.get(name) {
        Some(m) => Ok(m.clone()),
        None => Err(err!(ConfigurationFailure, format!("Module \"{}\" is not loaded.", name))),
    }
}

/// Retrieve a loaded expression binding
pub fn get_expression(name: &str) -> Result<Arc<dyn ExpressionBinding>> {
    let reader = EXPRESSIONS.lock();
    match reader.get(name) {
        Some(m) => Ok(m.clone()),
        None => Err(err!(
            ConfigurationFailure,
            format!("Expression provider \"{}\" is not loaded.", name)
        )),
    }
}

/// Manually bind a module to Katalyst. Generally the bind_katalyst! should be used instead.
pub fn bind_module(module: Arc<dyn ModuleProvider>) {
    let n = module.name();
    let mut writer = MODULES.lock();
    writer.insert(n.into(), module);
}

/// Manually bind an expression binding to Katalyst. Generally the bind_katalyst! should be used instead.
pub fn bind_expression(expr: Arc<dyn ExpressionBinding>) {
    let n = expr.identifier();
    let mut writer = EXPRESSIONS.lock();
    writer.insert(n.into(), expr);
}
