use crate::prelude::*;
use std::{fmt::Debug, sync::Arc};
use unstructured::Document;

lazy_static! {
    static ref DEF_STRING: String = String::default();
}

pub type RenderResult = Result<String>;
pub type ExpressionResult = Result<Document>;
pub type ExpressionArg = Arc<CompiledExpression>;
pub type ExpressionRenderMethod =
    Arc<Fn(&RequestContext, &[ExpressionArg]) -> ExpressionResult + Send + Sync>;

#[derive(Clone)]
pub enum ExpressionResultType {
    Text,
    Number,
    Boolean,
}

/// This is the trait used by Katalyst for building the placeholders used in a downstream URL template
pub trait ExpressionBinding: Send + Sync {
    /// The identifier in this template to locate that this provider should be used
    fn identifier(&self) -> &'static str;
    /// This returns the render function for this expression
    fn make_fn(&self, name: &str, args: &[ExpressionArg]) -> Result<ExpressionRenderMethod>;
}

/// This is the trait that must be implemented by any expression that can be compiled from config
pub trait CompiledExpression: Send + Sync + Debug {
    /// Render processes the compiled expression and returns a string rendering of the contents regardless of underlying types
    fn render(&self, guard: &RequestContext) -> RenderResult;

    /// Get the direct result of evaluating the expression
    fn result(&self, guard: &RequestContext) -> ExpressionResult;

    /// Return a document shell indicating the type
    fn result_type(&self) -> Document;
}
