use crate::expression::*;
use crate::prelude::*;

pub struct HeaderExpressionBuilder {}

impl ExpressionBuilder for HeaderExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "header"
    }

    fn build_placeholder(&self, value: String) -> Box<CompiledExpression> {
        Box::new(HeaderCompiledExpression { header: value })
    }
}

#[derive(Debug)]
struct HeaderCompiledExpression {
    header: String,
}

impl CompiledExpression for HeaderCompiledExpression {
    fn get_value(&self, ctx: &Context) -> String {
        match &ctx.upstream.request {
            Some(s) => match s.headers().get(&self.header) {
                Some(t) => t.to_str().unwrap_or_default().to_string(),
                None => self.none().to_string(),
            },
            None => self.none().to_string(),
        }
    }

    fn duplicate(&self) -> Box<CompiledExpression> {
        HeaderCompiledExpression {
            header: self.header.to_owned(),
        }
        .boxed()
    }
}
