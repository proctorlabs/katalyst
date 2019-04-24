use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;

binding! {
    Json {
        #[args(count=1)]
        fn value(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let key = args[0].render(ctx)?;
            println!("{}", key);
            let json: Arc<serde_json::Value> = ctx.get_extension_data()?;
            println!("{}", json);
            let res = &json[&key];
            println!("{}", res);
            Ok(res.as_str().ok_or_else(|| RequestFailure::Internal)?.to_string())
        };
    }
}
