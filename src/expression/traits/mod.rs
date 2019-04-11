use super::Expression;
use crate::prelude::*;
use std::fmt;
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
    /// Construct a new KatalystCompiledExpression to use in this config.
    /// Note that these are reused and should be immutable once made.
    fn build(&self, value: String) -> Arc<CompiledExpression>;

    fn make_fn(&self, args: &[ExpressionArg]) -> Result<ExpressionRenderMethod, KatalystError>;
}

pub struct CompiledExpressionImpl {
    raw: String,
    args: Expression,
    eval: ExpressionRenderMethod,
}

impl CompiledExpressionImpl {
    pub fn make(
        raw: String,
        args: Expression,
        eval: ExpressionRenderMethod,
    ) -> Arc<CompiledExpression> {
        Arc::new(CompiledExpressionImpl {
            raw: raw,
            args: args,
            eval: eval,
        })
    }
}

impl CompiledExpression for CompiledExpressionImpl {
    fn render(&self, ctx: &Context) -> String {
        (self.eval)(ctx, &self.args)
    }

    fn duplicate(&self) -> Arc<CompiledExpression> {
        Arc::new(CompiledExpressionImpl {
            raw: self.raw.to_string(),
            args: self.args.clone(),
            eval: self.eval.clone(),
        })
    }
}

impl Debug for CompiledExpressionImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expression {{ {} }}", self.raw)
    }
}

/// This provides the actual value replacement used in the downstream URL template
pub trait CompiledExpression: Send + Sync + Debug {
    /// Returns the string value that should be used as a replacement for this Placeholder in the pipeline context
    fn render(&self, ctx: &Context) -> String;

    /// Creates a boxed duplicate of this placeholder
    fn duplicate(&self) -> Arc<CompiledExpression>;

    /// Returned when no match is found for placeholder
    fn none(&self) -> &String {
        &DEF_STRING
    }
}

pub trait Expressable {
    fn render(&self, state: &Context) -> String;
}

impl Expressable for Expression {
    fn render(&self, state: &Context) -> String {
        let mut result = String::new();
        for part in self.iter() {
            result.push_str(&part.render(state));
        }
        result
    }
}
