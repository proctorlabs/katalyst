use crate::expression::*;
use crate::prelude::*;

pub struct ClaimExpressionBuilder {}

impl ExpressionBuilder for ClaimExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "claim"
    }

    fn build_placeholder(&self, value: String) -> Box<CompiledExpression> {
        Box::new(ClaimCompiledExpression { claim_key: value })
    }
}

#[derive(Debug)]
struct ClaimCompiledExpression {
    claim_key: String,
}

impl CompiledExpression for ClaimCompiledExpression {
    fn get_value(&self, ctx: &Context) -> String {
        if let Some(auth_info) = &ctx.detail.authentication {
            auth_info.get_claim(self.claim_key.to_string())
        } else {
            self.none().to_string()
        }
    }

    fn duplicate(&self) -> Box<CompiledExpression> {
        ClaimCompiledExpression {
            claim_key: self.claim_key.to_owned(),
        }
        .boxed()
    }
}
