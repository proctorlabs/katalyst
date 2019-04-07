use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

pub struct HttpExpressionBuilder {}

impl ExpressionBuilder for HttpExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "http"
    }

    fn build(&self, value: String) -> Arc<CompiledExpression> {
        match value.as_str() {
            "method" => Arc::new(HttpMethodCompiledExpression {}),
            "ip" => Arc::new(HttpIPCompiledExpression {}),
            "path" => Arc::new(HttpUriCompiledExpression {}),
            &_ => Arc::new(HttpMethodCompiledExpression {}),
        }
    }

    fn make_fn(
        &self,
        args: Vec<Arc<CompiledExpression>>,
    ) -> Result<ExpressionRenderFn, KatalystError> {
        Ok(Arc::new(|_, _| "".to_string()))
    }
}

#[derive(Debug)]
struct HttpMethodCompiledExpression {}
#[derive(Debug)]
struct HttpIPCompiledExpression {}
#[derive(Debug)]
struct HttpUriCompiledExpression {}
#[derive(Debug)]
struct HttpQueryCompiledExpression {}

impl CompiledExpression for HttpMethodCompiledExpression {
    fn render(&self, ctx: &Context) -> String {
        match &ctx.upstream.request {
            Some(s) => s.method().as_str().to_owned(),
            None => String::default(),
        }
    }

    fn duplicate(&self) -> Arc<CompiledExpression> {
        Arc::new(HttpMethodCompiledExpression {})
    }
}

impl CompiledExpression for HttpIPCompiledExpression {
    fn render(&self, ctx: &Context) -> String {
        ctx.remote_addr.ip().to_string()
    }

    fn duplicate(&self) -> Arc<CompiledExpression> {
        Arc::new(HttpIPCompiledExpression {})
    }
}

impl CompiledExpression for HttpUriCompiledExpression {
    fn render(&self, ctx: &Context) -> String {
        match &ctx.upstream.request {
            Some(s) => s.uri().path().to_string(),
            None => String::default(),
        }
    }

    fn duplicate(&self) -> Arc<CompiledExpression> {
        Arc::new(HttpUriCompiledExpression {})
    }
}

impl CompiledExpression for HttpQueryCompiledExpression {
    fn render(&self, ctx: &Context) -> String {
        match &ctx.upstream.request {
            Some(s) => s.uri().query().unwrap_or_default().to_string(),
            None => String::default(),
        }
    }

    fn duplicate(&self) -> Arc<CompiledExpression> {
        Arc::new(HttpQueryCompiledExpression {})
    }
}
