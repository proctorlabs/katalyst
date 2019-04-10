use super::Expression;
use crate::prelude::*;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

lazy_static! {
    static ref DEF_STRING: String = String::default();
}

pub type ExpressionRenderFn =
    Arc<Fn(&Context, &Vec<Arc<CompiledExpression>>) -> String + Send + Sync>;

/// This is the trait used by Katalyst for building the placeholders used in a downstream URL template
pub trait ExpressionBuilder: Send + Sync {
    /// The identifier in this template to locate that this provider should be used
    fn identifier(&self) -> &'static str;
    /// Construct a new KatalystCompiledExpression to use in this config.
    /// Note that these are reused and should be immutable once made.
    fn build(&self, value: String) -> Arc<CompiledExpression>;

    fn make_fn(
        &self,
        args: Vec<Arc<CompiledExpression>>,
    ) -> Result<ExpressionRenderFn, KatalystError>;
}

pub struct CompiledExpressionImpl {
    raw: String,
    args: Expression,
    eval: ExpressionRenderFn,
}

impl CompiledExpressionImpl {
    pub fn make(
        raw: String,
        args: Expression,
        eval: ExpressionRenderFn,
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

impl CompiledExpression for String {
    fn render(&self, _: &Context) -> String {
        self.to_string()
    }

    fn duplicate(&self) -> Arc<CompiledExpression> {
        Arc::new(self.to_owned())
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
