use crate::expression::*;
use crate::prelude::*;
use std::str::FromStr;

#[derive(ExpressionBinding)]
#[expression(name = "http", bind = method)]
#[expression(bind = ip)]
#[expression(bind = path)]
#[expression(bind = query)]
#[expression(bind = query_param)]
#[expression(bind = header)]
#[expression(bind = matched)]
pub struct Http;

impl Http {
    fn method(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(ctx.request.method().as_str().into())
    }

    fn ip(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(ctx.metadata.remote_ip.to_owned().into())
    }

    fn path(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(ctx.metadata.url.path().into())
    }

    fn query(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(ctx.metadata.url.query().unwrap_or_default().into())
    }

    fn query_param(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
        let name = args[0].render(ctx)?;
        let res = ctx.metadata.url.query_pairs().find(|q| q.0 == name);
        res.map_or_else(|| Err(GatewayError::InternalServerError), |v| Ok(v.1.to_string().into()))
    }

    fn header(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
        let hdr = ctx
            .request
            .header(&args[0].render(ctx)?)
            .ok_or_else(|| GatewayError::InternalServerError)?;
        Ok(hdr.into())
    }

    fn matched(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
        let value = args[0].render(ctx)?;
        let caps = &ctx.get_matched()?.captures;
        let res = caps.get(&value).ok_or_else(|| GatewayError::InternalServerError)?;
        Ok(String::from_str(res).map_err(|_| GatewayError::InternalServerError)?.to_string().into())
    }
}
