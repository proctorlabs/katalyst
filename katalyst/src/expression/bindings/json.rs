use crate::expression::*;
use crate::prelude::*;

binding! {
    Json {
        #[args(count=0)]
        fn value(_: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            Err(RequestFailure::Internal)
        };
    }
}
