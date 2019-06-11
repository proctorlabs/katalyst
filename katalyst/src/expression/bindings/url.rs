use crate::prelude::*;

#[derive(ExpressionBinding)]
#[expression(name = "url", bind = segment)]
#[expression(bind = all)]
pub struct Url;

impl Url {
    fn segment(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        let mut result = String::new();
        result.push_str(r"(?P<");
        result.push_str(&args[0].render(guard)?);
        result.push_str(r">[^/]+)");
        Ok(result.into())
    }

    fn all(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        let mut result = String::new();
        result.push_str(r"(?P<");
        result.push_str(&args[0].render(guard)?);
        result.push_str(r">.*)");
        Ok(result.into())
    }
}
