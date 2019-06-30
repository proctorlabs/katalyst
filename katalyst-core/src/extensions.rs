/*!
This module provides helpers for extending Katalyst functionality. Look at the
bind_katalyst! macro for a simple way to add extensions.
*/
use crate::{expression::ExpressionBinding, modules::ModuleProvider};
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    static ref MODULES: Mutex<HashMap<String, Arc<dyn ModuleProvider>>> = Default::default();
    static ref EXPRESSIONS: Mutex<HashMap<String, Arc<dyn ExpressionBinding>>> = Default::default();
}

/// Bind a module or expression to Katalyst
#[macro_export]
macro_rules! bind_katalyst {
    (@ $module:ty) => {
        katalyst::extensions::bind_module(std::sync::Arc::new($module::default()))
    };
    ($expression:ty) => {
        katalyst::extensions::bind_expression(std::sync::Arc::new($expression::default()))
    };
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
