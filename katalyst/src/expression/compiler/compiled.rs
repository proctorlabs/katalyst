use crate::expression::*;
use crate::prelude::*;
use std::fmt;
use std::sync::Arc;

pub struct CompiledExpressionNode {
    pub name: String,
    pub result: ExpressionResultType,
    pub args: Vec<Arc<CompiledExpression>>,
    pub render_fn: ExpressionRenderMethod,
}

impl CompiledExpression for CompiledExpressionNode {
    fn render(&self, ctx: &Context) -> ExpressionResult {
        (self.render_fn)(ctx, &self.args)
    }
}

impl CompiledExpression for String {
    fn render(&self, _: &Context) -> ExpressionResult {
        Ok(self.to_string())
    }
}

impl CompiledExpression for u64 {
    fn render(&self, _: &Context) -> ExpressionResult {
        Ok(self.to_string())
    }
}

impl CompiledExpression for bool {
    fn render(&self, _: &Context) -> ExpressionResult {
        Ok(self.to_string())
    }
}

impl fmt::Debug for CompiledExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(", &self.name)?;
        self.args.fmt(f)?;
        write!(f, ") -> ")?;
        match self.result {
            ExpressionResultType::Text => write!(f, "str"),
            ExpressionResultType::Number => write!(f, "u64"),
            ExpressionResultType::Boolean => write!(f, "bool"),
        }
    }
}
