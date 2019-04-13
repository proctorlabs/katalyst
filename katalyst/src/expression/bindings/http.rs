use crate::expression::*;
use crate::prelude::*;
use std::str::FromStr;

binding! {
    Http {
        #[args(count=0)]
        fn method(ctx: &Context, _: &[ExpressionArg]) -> String {
            if let Some(req) = &ctx.upstream.request {
                req.method().as_str().to_owned()
            } else {
                "".to_string()
            }
        };

        #[args(count=0)]
        fn ip(ctx: &Context, _: &[ExpressionArg]) -> String {
            ctx.remote_addr.ip().to_string()
        };

        #[args(count=0)]
        fn path(ctx: &Context, _: &[ExpressionArg]) -> String {
            if let Some(req) = &ctx.upstream.request {
                req.uri().path().to_string()
            } else {
                "".to_string()
            }
        };

        #[args(count=0)]
        fn query(ctx: &Context, _: &[ExpressionArg]) -> String {
            if let Some(req) = &ctx.upstream.request {
                req.uri().query().unwrap_or_default().to_string()
            } else {
                "".to_string()
            }
        };

        #[args(count=1)]
        fn header(ctx: &Context, args: &[ExpressionArg]) -> String {
            match &ctx.upstream.request {
                Some(s) => match s.headers().get(args[0].render(ctx)) {
                    Some(t) => t.to_str().unwrap_or_default().to_string(),
                    None => "".to_string(),
                },
                None => "".to_string(),
            }
        };

        #[args(count=1)]
        fn matched(ctx: &Context, args: &[ExpressionArg]) -> String {
            let value = args[0].render(ctx);
            match &ctx.detail.captures {
                Some(caps) => {
                    let res = caps.get(&value).unwrap();
                    String::from_str(res).unwrap_or_default().to_string()
                }
                None => "".to_string(),
            }
        };
    }
}
