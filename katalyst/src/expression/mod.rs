mod bindings;
mod compiler;
mod traits;

use crate::prelude::*;
use bindings::*;
pub use compiler::Compiler;
use std::sync::Arc;
pub use traits::*;

pub type ExpressionArgs = Vec<Arc<CompiledExpression>>;
pub type Expression = Vec<Arc<CompiledExpression>>;

impl CompiledExpression for Expression {
    fn render(&self, state: &Context) -> ExpressionResult {
        let mut result = String::new();
        for part in self.iter() {
            result.push_str(&part.render(state)?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_template() {
        let compiler = Compiler::default();
        compiler
            .compile_template(Some("/testing/the/parser/{{http.ip()}}/test"))
            .unwrap();
    }

}
