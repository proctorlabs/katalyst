use crate::prelude::*;
use std::sync::Arc;

pub type ExpressionRenderMethod =
    Arc<Fn(&Context, &Vec<Arc<CompiledExpressionNode>>) -> String + Send + Sync>;

pub enum ExpressionResultType {
    Text,
    Number,
    Boolean,
}

pub struct CompiledExpressionNode {
    pub result: ExpressionResultType,
    pub args: Vec<Arc<CompiledExpressionNode>>,
    pub render_fn: ExpressionRenderMethod,
}
