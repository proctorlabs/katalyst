use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

pub struct EnvExpressionBuilder {}

impl EnvExpressionBuilder {
    fn call(ctx: &Context, args: &[ExpressionArg]) -> String {
        std::env::var_os(args[0].render(ctx))
            .expect("Environment variable not set!")
            .to_str()
            .unwrap_or_default()
            .to_owned()
    }
}

impl ExpressionBuilder for EnvExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "env"
    }

    fn make_fn(&self, _: &[ExpressionArg]) -> Result<ExpressionRenderMethod, KatalystError> {
        Ok(Arc::new(EnvExpressionBuilder::call))
    }
}
