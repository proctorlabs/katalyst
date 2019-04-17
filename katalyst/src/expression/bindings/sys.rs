use crate::expression::*;
use crate::prelude::*;

binding! {
    Sys {
        #[args(count=1)]
        fn env(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            Ok(std::env::var_os(args[0].render(ctx)?)
                .ok_or_else(|| RequestFailure::Internal)?
                .to_str()
                .ok_or_else(|| RequestFailure::Internal)?
                .to_owned())
        };
    }
}
