use crate::expression::*;
use crate::prelude::*;

binding! {
    Url {
        #[args(count=1)]
        fn segment(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let mut result = String::new();
            result.push_str(r"(?P<");
            result.push_str(&args[0].render(ctx)?);
            result.push_str(r">[^/]+)");
            Ok(result.into())
        };

        #[args(count=1)]
        fn all(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let mut result = String::new();
            result.push_str(r"(?P<");
            result.push_str(&args[0].render(ctx)?);
            result.push_str(r">.*)");
            Ok(result.into())
        };
    }
}
