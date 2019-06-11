use crate::prelude::*;

#[derive(ExpressionBinding)]
#[expression(name = "sys", bind = env)]
pub struct Sys;

impl Sys {
    fn env(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        let env_var = args[0].render(guard)?;
        Ok(std::env::var_os(&env_var)
            .ok_or_else(|| fail!(_ INTERNAL_SERVER_ERROR, format!("Environment var {} not found!", &env_var)))?
            .to_str()
            .ok_or_else(|| fail!(_ INTERNAL_SERVER_ERROR, format!("Environment var {} is of invalid format!", &env_var)))?
            .into())
    }
}
