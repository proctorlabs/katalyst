use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

pub struct HttpExpressionBuilder {}

impl HttpExpressionBuilder {
    fn call(ctx: &Context, args: &[ExpressionArg]) -> String {
        if let Some(req) = &ctx.upstream.request {
            let value = args[0].render(ctx);
            match value.as_str() {
                "method" => req.method().as_str().to_owned(),
                "ip" => ctx.remote_addr.ip().to_string(),
                "path" => req.uri().path().to_string(),
                "query" => req.uri().query().unwrap_or_default().to_string(),
                &_ => req.method().as_str().to_owned(),
            }
        } else {
            "".to_string()
        }
    }
}

impl ExpressionBuilder for HttpExpressionBuilder {
    fn identifier(&self) -> &'static str {
        "http"
    }

    fn make_fn(&self, _: &[ExpressionArg]) -> Result<ExpressionRenderMethod, KatalystError> {
        Ok(Arc::new(HttpExpressionBuilder::call))
    }
}
