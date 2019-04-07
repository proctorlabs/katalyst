use crate::expression::*;
use crate::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub struct RegexExpressionBuilder {}

impl ExpressionBuilder for RegexExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "regex"
    }

    fn build(&self, value: String) -> Arc<CompiledExpression> {
        Arc::new(RegexCompiledExpression { val: value })
    }

    fn make_fn(
        &self,
        args: Vec<Arc<CompiledExpression>>,
    ) -> Result<ExpressionRenderFn, KatalystError> {
        Ok(Arc::new(|_, _| "".to_string()))
    }
}

#[derive(Debug)]
struct RegexCompiledExpression {
    val: String,
}

impl CompiledExpression for RegexCompiledExpression {
    fn render(&self, ctx: &Context) -> String {
        match &ctx.detail.captures {
            Some(caps) => {
                let res = caps.get(&self.val).unwrap_or_else(|| self.none());
                String::from_str(res).unwrap_or_default().to_string()
            }
            None => self.none().to_string(),
        }
    }

    fn duplicate(&self) -> Arc<CompiledExpression> {
        Arc::new(RegexCompiledExpression {
            val: self.val.to_owned(),
        })
    }
}
