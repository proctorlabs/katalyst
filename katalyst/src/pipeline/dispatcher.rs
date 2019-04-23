use crate::prelude::*;
use crate::*;

pub fn dispatch(ctx: Context) -> ModuleResult {
    let r = try_fut!(ctx.detail.route()).clone();
    Box::new(r.handler.dispatch(ctx))
}
