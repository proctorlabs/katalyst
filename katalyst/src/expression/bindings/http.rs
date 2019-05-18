use crate::expression::*;
use crate::prelude::*;
use std::str::FromStr;

binding! {
    Http {
        #[args(count=0)]
        fn method(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            Ok(ctx.request.raw().method().as_str().into())
        };

        #[args(count=0)]
        fn ip(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            Ok(ctx.detail.remote_ip.to_owned().into())
        };

        #[args(count=0)]
        fn path(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            Ok(ctx.detail.url.path().into())
        };

        #[args(count=0)]
        fn query(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            Ok(ctx.detail.url.query().unwrap_or_default().into())
        };

        #[args(count=1)]
        fn query_param(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let name = args[0].render(ctx)?;
            let res = ctx.detail.url.query_pairs().find(|q| q.0 == name);
            res.map_or_else(|| Err(GatewayError::InternalServerError), |v| Ok(v.1.to_string().into()))
        };

        #[args(count=1)]
        fn header(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let hdr = ctx.request.raw().headers().get(args[0].render(ctx)?).ok_or_else(|| GatewayError::InternalServerError)?;
            Ok(hdr.to_str().map_err(|_| GatewayError::InternalServerError)?.into())
        };

        #[args(count=1)]
        fn matched(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let value = args[0].render(ctx)?;
            match &ctx.detail.captures {
                Some(caps) => Ok({
                    let res = caps.get(&value).ok_or_else(|| GatewayError::InternalServerError)?;
                    String::from_str(res)
                        .map_err(|_| GatewayError::InternalServerError)?
                        .to_string()
                }.into()),

                None => Err(GatewayError::InternalServerError),
            }
        };
    }
}
