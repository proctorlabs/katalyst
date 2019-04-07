use super::Expression;
use crate::prelude::*;
use std::fmt::Debug;

lazy_static! {
    static ref DEF_STRING: String = String::default();
}

/// This is the trait used by Katalyst for building the placeholders used in a downstream URL template
pub trait ExpressionBuilder: Send + Sync {
    /// The identifier in this template to locate that this provider should be used
    fn identifier(&self) -> &'static str;
    /// Construct a new KatalystCompiledExpression to use in this config.
    /// Note that these are reused and should be immutable once made.
    fn build_placeholder(&self, value: String) -> Box<CompiledExpression>;
}

/// This provides the actual value replacement used in the downstream URL template
pub trait CompiledExpression: Send + Sync + Debug {
    /// Returns the string value that should be used as a replacement for this Placeholder in the pipeline context
    fn get_value(&self, state: &Context) -> String;

    /// Creates a boxed duplicate of this placeholder
    fn duplicate(&self) -> Box<CompiledExpression>;

    /// Returned when no match is found for placeholder
    fn none(&self) -> &String {
        &DEF_STRING
    }

    fn boxed(self) -> Box<CompiledExpression>
    where
        Self: std::marker::Sized,
        Self: 'static,
    {
        Box::new(self)
    }
}

impl CompiledExpression for String {
    fn get_value(&self, _: &Context) -> String {
        self.to_string()
    }

    fn duplicate(&self) -> Box<CompiledExpression> {
        Box::new(self.to_owned())
    }
}

pub trait Expressable {
    fn get_value(&self, state: &Context) -> String;
}

impl Expressable for Expression {
    fn get_value(&self, state: &Context) -> String {
        let mut result = String::new();
        for part in self.iter() {
            result.push_str(&part.get_value(state));
        }
        result
    }
}
