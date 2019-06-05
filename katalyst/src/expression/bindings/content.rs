use crate::{expression::*, prelude::*};
use unstructured::Document;

#[derive(ExpressionBinding)]
#[expression(name = "content", bind = val)]
pub struct Content;

impl Content {
    fn val(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        let key = args[0].render(guard)?;
        let key = Document::String(key);
        let req = guard.get_http_request()?;
        let http_req: &HttpRequest = &req;
        if let HttpRequest::ParsedRequest(d) = http_req {
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
