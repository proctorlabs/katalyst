use super::PrecomputedExpression;
use crate::expression::*;

pub struct EnvExpressionBuilder {}

impl ExpressionBuilder for EnvExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "env"
    }

    fn build_placeholder(&self, value: String) -> Box<CompiledExpression> {
        PrecomputedExpression::make(
            std::env::var_os(value)
                .expect("Environment variable not set!")
                .to_str()
                .unwrap_or_default()
                .to_owned(),
        )
    }
}