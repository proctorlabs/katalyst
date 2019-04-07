use crate::expression::*;
use crate::prelude::*;
use std::str::FromStr;

pub struct RegexExpressionBuilder {}

impl ExpressionBuilder for RegexExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "regex"
    }

    fn build_placeholder(&self, value: String) -> Box<CompiledExpression> {
        Box::new(RegexCompiledExpression { val: value })
    }
}

#[derive(Debug)]
struct RegexCompiledExpression {
    val: String,
}

impl CompiledExpression for RegexCompiledExpression {
    fn get_value(&self, ctx: &Context) -> String {
        match &ctx.detail.captures {
            Some(caps) => {
                let res = caps.get(&self.val).unwrap_or_else(|| self.none());
                String::from_str(res).unwrap_or_default().to_string()
            }
            None => self.none().to_string(),
        }
    }

    fn duplicate(&self) -> Box<CompiledExpression> {
        RegexCompiledExpression {
            val: self.val.to_owned(),
        }
        .boxed()
    }
}
