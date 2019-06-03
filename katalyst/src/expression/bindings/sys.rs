use crate::{expression::*, prelude::*};

#[derive(ExpressionBinding)]
#[expression(name = "sys", bind = env)]
pub struct Sys;

impl Sys {
    fn env(guard: &ContextGuard, args: &[ExpressionArg]) -> ExpressionResult {
        Ok(std::env::var_os(args[0].render(guard)?)
            .ok_or_else(|| GatewayError::InternalServerError)?
            .to_str()
            .ok_or_else(|| GatewayError::InternalServerError)?
            .into())
    }
}
