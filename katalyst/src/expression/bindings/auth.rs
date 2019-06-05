use crate::{expression::*, prelude::*};

#[derive(ExpressionBinding)]
#[expression(name = "auth", bind = claim)]
pub struct Auth;

impl Auth {
    fn claim(guard: &RequestContext, args: &[ExpressionArg]) -> ExpressionResult {
        Ok(guard.get_authentication()?.get_claim(args[0].render(guard)?).into())
    }
}
