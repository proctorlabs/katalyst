use crate::expression::*;
use crate::prelude::*;

binding! {
    Sys {
        #[args(count=1)]
        fn env(ctx: &Context, args: &[ExpressionArg]) -> String {
            std::env::var_os(args[0].render(ctx))
            .expect("Environment variable not set!")
            .to_str()
            .unwrap_or_default()
            .to_owned()
        };
    }
}
