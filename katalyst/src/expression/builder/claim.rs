use crate::expression::*;
use crate::prelude::*;

binding! {
    Auth {
        #[args(count=1)]
        fn claim(ctx: &Context, args: &[ExpressionArg]) -> String {
            if let Some(auth_info) = &ctx.detail.authentication {
                auth_info.get_claim(args[0].render(&ctx))
            } else {
                "".to_string()
            }
        };
    }
}
