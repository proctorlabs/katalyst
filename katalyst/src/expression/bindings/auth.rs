use crate::{expression::*, prelude::*};

#[derive(ExpressionBinding)]
#[expression(name = "auth", bind = claim)]
pub struct Auth;

impl Auth {
    fn claim(ctx: &Context, args: &[ExpressionArg]) -> ExpressionResult {
        Ok(ctx.get_authenticated()?.detail.get_claim(args[0].render(&ctx)?).into())
    }
}
