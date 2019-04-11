use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

pub struct HeaderExpressionBuilder {}

impl HeaderExpressionBuilder {
    fn call(ctx: &Context, args: &[ExpressionArg]) -> String {
        match &ctx.upstream.request {
            Some(s) => match s.headers().get(args[0].render(ctx)) {
                Some(t) => t.to_str().unwrap_or_default().to_string(),
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }
}

impl ExpressionBuilder for HeaderExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "header"
    }

    fn make_fn(&self, _: &[ExpressionArg]) -> Result<ExpressionRenderMethod, KatalystError> {
        Ok(Arc::new(HeaderExpressionBuilder::call))
    }
}
