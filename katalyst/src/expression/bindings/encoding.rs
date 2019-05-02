use crate::expression::*;
use crate::prelude::*;
use base64::{decode, encode};
use std::str;

binding! {
    Encode {
        fn base64(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let to_encode = args[0].render(ctx)?;
            Ok(encode(&to_encode).into())
        };
    }
}

binding! {
    Decode {
        fn base64(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
            let to_decode = args[0].render(ctx)?;
            Ok(str::from_utf8(decode(&to_decode)
                .map_err(|_|RequestFailure::Internal)?
                    .as_slice())
                .map_err(|_|RequestFailure::Internal)?
                    .to_string().into())
        };
    }
}
