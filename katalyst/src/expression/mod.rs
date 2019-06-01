/*!
Katalyst configuration is based around expressions. Expressions are a syntax for templating and
customization from within the configuration file.
*/

mod bindings;
pub(crate) mod compiler;
mod traits;

use crate::prelude::*;
use bindings::*;
pub use compiler::Compiler;
use std::sync::Arc;
pub use traits::*;
use unstructured::Document;

pub type ExpressionArgs = Vec<Arc<CompiledExpression>>;
pub type Expression = Vec<Arc<CompiledExpression>>;

impl CompiledExpression for Expression {
    fn render(&self, state: &Context) -> RenderResult {
        let mut result = String::new();
        for part in self.iter() {
            result.push_str(&part.render(state)?);
        }
        Ok(result)
    }

    fn result(&self, ctx: &Context) -> ExpressionResult {
        let mut res = vec![];
        for exp in self.iter() {
            res.push(exp.result(ctx)?);
        }
        Ok(res.into())
    }

    fn result_type(&self) -> Document {
        Document::Seq(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_template() {
        let compiler = Compiler::default();
        compiler.compile_template(Some("/testing/the/parser/{{http.ip()}}/test")).unwrap();
    }

}
