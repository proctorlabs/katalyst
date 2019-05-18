use crate::expression::*;
use crate::prelude::*;
use std::sync::Arc;
use unstructured::Document;

binding! {
    Content {
        #[args(count=1)]
        fn val(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let key = args[0].render(ctx)?;
            let key = Document::String(key);
            let val: Arc<Document> = ctx.get_extension_data()?;
            let res = match val.as_ref() {
                Document::Map(map) => Ok(map.get(&key).ok_or(GatewayError::InternalServerError)?),
                _ => Err(GatewayError::InternalServerError)
            }?;
            match res {
                Document::String(s) => Ok(s.to_owned().into()),
                _ => Err(GatewayError::InternalServerError)
            }
        };
    }
}
