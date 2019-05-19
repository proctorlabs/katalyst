use crate::expression::*;
use crate::prelude::*;

#[derive(ExpressionBinding)]
#[allow(dead_code)]
pub enum Sys {
    #[expression(method=env)]
    Env,
}

impl Sys {
    fn env(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
        println!("test");
        Ok(std::env::var_os(args[0].render(ctx)?)
            .ok_or_else(|| GatewayError::InternalServerError)?
            .to_str()
            .ok_or_else(|| GatewayError::InternalServerError)?
            .into())
    }
}
