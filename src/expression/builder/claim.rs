use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

pub struct ClaimExpressionBuilder {}

impl ExpressionBuilder for ClaimExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "claim"
    }

    fn build(&self, value: String) -> Arc<CompiledExpression> {
        Arc::new(ClaimCompiledExpression { claim_key: value })
    }

    fn make_fn(
        &self,
        args: Vec<Arc<CompiledExpression>>,
    ) -> Result<ExpressionRenderFn, KatalystError> {
        Ok(Arc::new(|_, _| "".to_string()))
    }
}

#[derive(Debug)]
struct ClaimCompiledExpression {
    claim_key: String,
}

impl CompiledExpression for ClaimCompiledExpression {
    fn render(&self, ctx: &Context) -> String {
        if let Some(auth_info) = &ctx.detail.authentication {
            auth_info.get_claim(self.claim_key.to_string())
        } else {
            self.none().to_string()
        }
    }

    fn duplicate(&self) -> Arc<CompiledExpression> {
        Arc::new(ClaimCompiledExpression {
            claim_key: self.claim_key.to_owned(),
        })
    }
}
