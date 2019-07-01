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
    static ref INITIALIZED: bool = {
        crate::modules::initialize();
        crate::expression::initialize();
        true
    };
}

/// Bind a module or expression to Katalyst
#[macro_export]
macro_rules! bind_katalyst {
    (@ $( $module:expr ),* ) => {{
        $( $crate::extensions::bind_module(std::sync::Arc::new($module)); )*
    }};
    ( $( $expression:expr ),* ) => {
        $( $crate::extensions::bind_expression(std::sync::Arc::new($expression)); )*
    };
}

pub(crate) fn get_module(name: &str) -> Result<Arc<dyn ModuleProvider>> {
    if !*INITIALIZED {
        unreachable!();
    }
    let reader = MODULES.lock();
    match reader.get(name) {
        Some(m) => Ok(m.clone()),
        None => Err(err!(ConfigurationFailure, format!("Module \"{}\" is not loaded.", name))),
    }
}

pub(crate) fn get_expression(name: &str) -> Result<Arc<dyn ExpressionBinding>> {
    if !*INITIALIZED {
        unreachable!();
    }
    let reader = EXPRESSIONS.lock();
    match reader.get(name) {
        Some(m) => Ok(m.clone()),
        None => Err(err!(
            ConfigurationFailure,
            format!("Expression provider \"{}\" is not loaded.", name)
        )),
    }
}

/// Manually bind a module to Katalyst. If the module implements Default, then
/// the bind_katalyst! macro should be preferred.
pub fn bind_module(module: Arc<dyn ModuleProvider>) {
    let n = module.name();
    let mut writer = MODULES.lock();
    writer.insert(n.into(), module);
}

/// Manually bind an expression binding to Katalyst. If the expression implements Default,
/// then the bind_katalyst! macro should be preferred.
pub fn bind_expression(expr: Arc<dyn ExpressionBinding>) {
    let n = expr.identifier();
    let mut writer = EXPRESSIONS.lock();
    writer.insert(n.into(), expr);
}
