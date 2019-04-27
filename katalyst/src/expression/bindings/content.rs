use crate::expression::*;
use crate::prelude::*;
use serde_value::Value;
use std::sync::Arc;

binding! {
    Content {
        #[args(count=1)]
        fn val(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let key = args[0].render(ctx)?;
            let key = Value::String(key);
            let val: Arc<Value> = ctx.get_extension_data()?;
            let res = match val.as_ref() {
                Value::Map(map) => Ok(map.get(&key).ok_or(RequestFailure::Internal)?),
                _ => Err(RequestFailure::Internal)
            }?;
            match res {
                Value::String(s) => Ok(s.to_owned()),
                _ => Err(RequestFailure::Internal)
            }
        };
    }
}
