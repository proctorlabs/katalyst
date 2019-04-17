use crate::expression::*;
use crate::prelude::*;
use std::str::FromStr;

binding! {
    Http {
        #[args(count=0)]
        fn method(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            Ok(ctx.request()?.method().as_str().to_owned())
        };

        #[args(count=0)]
        fn ip(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            Ok(ctx.remote_addr.ip().to_string())
        };

        #[args(count=0)]
        fn path(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            Ok(ctx.request()?.uri().path().to_string())
        };

        #[args(count=0)]
        fn query(ctx: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            let req = ctx.request()?;
            Ok(req.uri().query().unwrap_or_default().to_string())
        };

        #[args(count=1)]
        fn query_param(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let req = ctx.request()?;
            let mut path = String::new();
            let uri = &req.uri();
            path.push_str(&uri.scheme_str().unwrap_or("http"));
            path.push_str("://");
            path.push_str(&uri.host().unwrap_or("localhost"));
            path.push_str(&req.uri().to_string());
            let name = args[0].render(ctx)?;
            let result = url::Url::parse(&path).map_err(|_| RequestFailure::Internal)?;
            let res = result.query_pairs().find(|q| q.0 == name);
            res.map_or_else(|| Err(RequestFailure::Internal), |v| Ok(v.1.to_string()))
        };

        #[args(count=1)]
        fn header(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let req = ctx.request()?;
            let hdr = req.headers().get(args[0].render(ctx)?).ok_or_else(|| RequestFailure::Internal)?;
            Ok(hdr.to_str().map_err(|_| RequestFailure::Internal)?.to_string())
        };

        #[args(count=1)]
        fn matched(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let value = args[0].render(ctx)?;
            match &ctx.detail.captures {
                Some(caps) => Ok({
                    let res = caps.get(&value).ok_or_else(|| RequestFailure::Internal)?;
                    String::from_str(res)
                        .map_err(|_| RequestFailure::Internal)?
                        .to_string()
                }),

                None => Err(RequestFailure::Internal),
            }
        };
    }
}
