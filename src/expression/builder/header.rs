use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

pub struct HeaderExpressionBuilder {}

impl ExpressionBuilder for HeaderExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "header"
    }

    fn build(&self, value: String) -> Arc<CompiledExpression> {
        Arc::new(HeaderCompiledExpression { header: value })
    }

    fn make_fn(
        &self,
        _args: Vec<Arc<CompiledExpression>>,
    ) -> Result<ExpressionRenderMethod, KatalystError> {
        Ok(Arc::new(|_, _| "".to_string()))
    }
}

#[derive(Debug)]
struct HeaderCompiledExpression {
    header: String,
}

impl CompiledExpression for HeaderCompiledExpression {
    fn render(&self, ctx: &Context) -> String {
        match &ctx.upstream.request {
            Some(s) => match s.headers().get(&self.header) {
                Some(t) => t.to_str().unwrap_or_default().to_string(),
                None => self.none().to_string(),
            },
            None => self.none().to_string(),
        }
    }

    fn duplicate(&self) -> Arc<CompiledExpression> {
        Arc::new(HeaderCompiledExpression {
            header: self.header.to_owned(),
        })
    }
}
