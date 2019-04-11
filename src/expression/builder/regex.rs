use crate::expression::*;
use crate::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub struct RegexExpressionBuilder {}

impl RegexExpressionBuilder {
    fn call(ctx: &Context, args: &[ExpressionArg]) -> String {
        let value = args[0].render(ctx);
        match &ctx.detail.captures {
            Some(caps) => {
                let res = caps.get(&value).unwrap();
                String::from_str(res).unwrap_or_default().to_string()
            }
            None => "".to_string(),
        }
    }
}

impl ExpressionBuilder for RegexExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "regex"
    }

    fn make_fn(&self, _: &[ExpressionArg]) -> Result<ExpressionRenderMethod, KatalystError> {
        Ok(Arc::new(RegexExpressionBuilder::call))
    }
}
