use crate::prelude::*;
use std::fmt::Debug;
use std::sync::Arc;

lazy_static! {
    static ref DEF_STRING: String = String::default();
}

pub type ExpressionArg = Arc<CompiledExpression>;
pub type ExpressionRenderMethod = Arc<Fn(&Context, &[ExpressionArg]) -> String + Send + Sync>;

#[derive(Clone)]
pub enum ExpressionResultType {
    Text,
    Number,
    Boolean,
}

/// This is the trait used by Katalyst for building the placeholders used in a downstream URL template
pub trait ExpressionBuilder: Send + Sync {
    /// The identifier in this template to locate that this provider should be used
    fn identifier(&self) -> &'static str;
    /// This returns the render function for this expression
    fn make_fn(&self, args: &[ExpressionArg]) -> Result<ExpressionRenderMethod, KatalystError>;
}

/// This provides the actual value replacement used in the downstream URL template
pub trait CompiledExpression: Send + Sync + Debug {
    /// Returns the string value that should be used as a replacement for this Placeholder in the pipeline context
    fn render(&self, ctx: &Context) -> String;
}
