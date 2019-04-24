use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

binding! {
    Json {
        #[args(count=1)]
        fn value(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let key = args[0].render(ctx)?;
            let json: Arc<serde_json::Value> = ctx.get_extension_data()?;
            let res = &json[&key];
            Ok(res.as_str().ok_or_else(|| RequestFailure::Internal)?.to_string())
        };
    }
}
