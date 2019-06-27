use crate::prelude::*;
use base64::{decode, encode};
use std::str;

#[derive(ExpressionBinding)]
#[expression(name = "encode", bind = base64)]
pub struct Encode;

impl Encode {
    fn base64(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        let to_encode = args[0].render(guard)?;
        Ok(encode(&to_encode).into())
    }
}

#[derive(ExpressionBinding)]
#[expression(name = "decode", bind = base64)]
pub struct Decode;

impl Decode {
    fn base64(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        let to_decode = args[0].render(guard)?;
        Ok(str::from_utf8(
            decode(&to_decode)
                .map_err(|e| fail!(_ INTERNAL_SERVER_ERROR, "Could not load data as utf8", e))?
                .as_slice(),
        )
        .map_err(|e| fail!(_ INTERNAL_SERVER_ERROR, "Could not decode base64 data", e))?
        .to_string()
        .into())
    }
}
