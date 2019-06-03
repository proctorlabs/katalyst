use crate::{expression::*, prelude::*};
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
    fn method(guard: &ContextGuard, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.method().as_str().into())
    }

    fn ip(guard: &ContextGuard, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.metadata()?.remote_ip.to_owned().into())
    }

    fn path(guard: &ContextGuard, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.metadata()?.url.path().into())
    }

    fn query(guard: &ContextGuard, _: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.metadata()?.url.query().unwrap_or_default().into())
    }

    fn query_param(guard: &ContextGuard, args: &[ExpressionArg]) -> ExpressionResult {
        let metadata = guard.metadata()?;
        let name = args[0].render(guard)?;
        let res = metadata.url.query_pairs().find(|q| q.0 == name);
        res.map_or_else(|| Err(GatewayError::InternalServerError), |v| Ok(v.1.to_string().into()))
    }

    fn header(guard: &ContextGuard, args: &[ExpressionArg]) -> ExpressionResult {
        let hdr = guard
            .header(&args[0].render(guard)?)
            .ok_or_else(|| GatewayError::InternalServerError)?;
        Ok(hdr.into())
    }

    fn matched(guard: &ContextGuard, args: &[ExpressionArg]) -> ExpressionResult {
        let value = args[0].render(guard)?;
        let caps = &guard.get_matched()?.captures;
        let res = caps.get(&value).ok_or_else(|| GatewayError::InternalServerError)?;
        Ok(String::from_str(res).map_err(|_| GatewayError::InternalServerError)?.to_string().into())
    }
}
