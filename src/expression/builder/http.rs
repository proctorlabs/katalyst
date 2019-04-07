use crate::expression::*;
use crate::prelude::*;

pub struct HttpExpressionBuilder {}

impl ExpressionBuilder for HttpExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "http"
    }

    fn build_placeholder(&self, value: String) -> Box<CompiledExpression> {
        match value.as_str() {
            "method" => HttpMethodCompiledExpression {}.boxed(),
            "ip" => HttpIPCompiledExpression {}.boxed(),
            "path" => HttpUriCompiledExpression {}.boxed(),
            &_ => HttpMethodCompiledExpression {}.boxed(),
        }
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
    fn get_value(&self, ctx: &Context) -> String {
        match &ctx.upstream.request {
            Some(s) => s.method().as_str().to_owned(),
            None => String::default(),
        }
    }

    fn duplicate(&self) -> Box<CompiledExpression> {
        HttpMethodCompiledExpression {}.boxed()
    }
}

impl CompiledExpression for HttpIPCompiledExpression {
    fn get_value(&self, ctx: &Context) -> String {
        ctx.remote_addr.ip().to_string()
    }

    fn duplicate(&self) -> Box<CompiledExpression> {
        HttpIPCompiledExpression {}.boxed()
    }
}

impl CompiledExpression for HttpUriCompiledExpression {
    fn get_value(&self, ctx: &Context) -> String {
        match &ctx.upstream.request {
            Some(s) => s.uri().path().to_string(),
            None => String::default(),
        }
    }

    fn duplicate(&self) -> Box<CompiledExpression> {
        HttpUriCompiledExpression {}.boxed()
    }
}

impl CompiledExpression for HttpQueryCompiledExpression {
    fn get_value(&self, ctx: &Context) -> String {
        match &ctx.upstream.request {
            Some(s) => s.uri().query().unwrap_or_default().to_string(),
            None => String::default(),
        }
    }

    fn duplicate(&self) -> Box<CompiledExpression> {
        HttpQueryCompiledExpression {}.boxed()
    }
}
