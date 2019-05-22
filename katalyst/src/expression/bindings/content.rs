use crate::expression::*;
use crate::prelude::*;
use unstructured::Document;

#[derive(ExpressionBinding)]
#[expression(name = "content", bind = val)]
pub struct Content;

impl Content {
    fn val(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
        let key = args[0].render(ctx)?;
        let key = Document::String(key);
        if let RequestContainer::ParsedRequest(d) = &ctx.request {
            let val = &d.2;
            let res = match val {
                Document::Map(map) => Ok(map.get(&key).ok_or(GatewayError::InternalServerError)?),
                _ => Err(GatewayError::InternalServerError),
            }?;
            match res {
                Document::String(s) => Ok(s.to_owned().into()),
                _ => Err(GatewayError::InternalServerError),
            }
        } else {
            return Err(GatewayError::InternalServerError);
        }
    }
}
