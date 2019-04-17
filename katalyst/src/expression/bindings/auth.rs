use crate::expression::*;
use crate::prelude::*;

binding! {
    Auth {
        #[args(count=1)]
        fn claim(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            if let Some(auth_info) = &ctx.detail.authentication {
                Ok(auth_info.get_claim(args[0].render(&ctx)?))
            } else {
                Err(RequestFailure::Internal)
            }
        };
    }
}
