use super::PrecomputedExpression;
use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

pub struct EnvExpressionBuilder {}

impl ExpressionBuilder for EnvExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "env"
    }

    fn build(&self, value: String) -> Arc<CompiledExpression> {
        PrecomputedExpression::make(
            std::env::var_os(value)
                .expect("Environment variable not set!")
                .to_str()
                .unwrap_or_default()
                .to_owned(),
        )
    }

    fn make_fn(
        &self,
        _args: Vec<Arc<CompiledExpression>>,
    ) -> Result<ExpressionRenderFn, KatalystError> {
        Ok(Arc::new(|_, _| "".to_string()))
    }
}
