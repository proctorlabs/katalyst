use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

pub struct ClaimExpressionBuilder {}

impl ClaimExpressionBuilder {
    fn call(ctx: &Context, args: &[ExpressionArg]) -> String {
        if let Some(auth_info) = &ctx.detail.authentication {
            auth_info.get_claim(args[0].render(&ctx))
        } else {
            "".to_string()
        }
    }
}

impl ExpressionBuilder for ClaimExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "claim"
    }

    fn make_fn(&self, _: &[ExpressionArg]) -> Result<ExpressionRenderMethod, KatalystError> {
        Ok(Arc::new(ClaimExpressionBuilder::call))
    }
}
