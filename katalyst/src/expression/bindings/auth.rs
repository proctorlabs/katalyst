use crate::{expression::*, prelude::*};

#[derive(ExpressionBinding)]
#[expression(name = "auth", bind = claim)]
pub struct Auth;

impl Auth {
    fn claim(guard: &ContextGuard, args: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.get_authenticated()?.detail.get_claim(args[0].render(guard)?).into())
    }
}
