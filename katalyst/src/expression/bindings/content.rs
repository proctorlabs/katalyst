use crate::prelude::*;
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
                Document::Map(map) => {
                    Ok(map.get(&key).ok_or_else(|| fail!(_ INTERNAL_SERVER_ERROR, format!("Incoming request does not container key {}", key)))?)
                }
                _ => fail!(INTERNAL_SERVER_ERROR, "Incoming request malformed"),
            }?;
            match res {
                Document::String(s) => Ok(s.to_owned().into()),
                _ => fail!(
                    INTERNAL_SERVER_ERROR,
                    format!("Key {}'s value was not able to be displayed", key)
                ),
            }
        } else {
            fail!(=> INTERNAL_SERVER_ERROR, "Request contents not loaded")
        }
    }
}
