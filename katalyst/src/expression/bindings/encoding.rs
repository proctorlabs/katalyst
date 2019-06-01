use crate::expression::*;
use crate::prelude::*;
use base64::{decode, encode};
use std::str;

#[derive(ExpressionBinding)]
#[expression(name = "encode", bind = base64)]
pub struct Encode;

impl Encode {
    fn base64(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
        let to_encode = args[0].render(ctx)?;
        Ok(encode(&to_encode).into())
    }
}

#[derive(ExpressionBinding)]
#[expression(name = "decode", bind = base64)]
pub struct Decode;

impl Decode {
    fn base64(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
        let to_decode = args[0].render(ctx)?;
        Ok(str::from_utf8(
            decode(&to_decode).map_err(|_| GatewayError::InternalServerError)?.as_slice(),
        )
        .map_err(|_| GatewayError::InternalServerError)?
        .to_string()
        .into())
    }
}
