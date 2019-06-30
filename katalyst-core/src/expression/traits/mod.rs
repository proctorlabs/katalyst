use crate::prelude::*;
use std::{fmt::Debug, sync::Arc};
use unstructured::Document;

lazy_static! {
    static ref DEF_STRING: String = String::default();
}

/// The result type of rendering an expression
pub type RenderResult = Result<String>;
/// The result type of calling an expression
pub type ExpressionResult = Result<Document>;
/// A single expression argument
pub type ExpressionArg = Arc<CompiledExpression>;
/// The method on an expression used to render the result
pub type ExpressionRenderMethod =
    Arc<Fn(&RequestContext, &[ExpressionArg]) -> ExpressionResult + Send + Sync>;

/// Metadata indicating the type of result from this expression
#[derive(Clone)]
pub enum ExpressionResultType {
    /// String result
    Text,
    /// Numeric result
    Number,
    /// Boolean result
    Boolean,
}

/// This is the trait used by KatalystCore for building the placeholders used in a downstream URL template
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
