use crate::expression::*;
use crate::prelude::*;

binding! {
    Env {
        #[args(count=1)]
        fn get(ctx: &Context, args: &[ExpressionArg]) -> String {
            std::env::var_os(args[0].render(ctx))
            .expect("Environment variable not set!")
            .to_str()
            .unwrap_or_default()
            .to_owned()
        };
    }
}
