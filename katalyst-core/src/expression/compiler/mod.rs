mod compiled;
pub(crate) mod nodes;

use crate::expression::*;
use compiled::*;
use std::collections::HashMap;

/// This is used for compiling expression bindings
pub struct Compiler;

impl Compiler {
    pub(crate) fn compile_template_map(
        template: &Option<HashMap<String, String>>,
    ) -> Result<Option<HashMap<String, Expression>>> {
        match template {
            Some(m) => Ok(Some({
                let mut result = HashMap::<String, Expression>::new();
                for i in m {
                    result.insert(i.0.to_string(), Compiler::compile_template(Some(i.1))?);
                }
                result
            })),
            None => Ok(None),
        }
    }

    pub(crate) fn compile_template_option(template: Option<&str>) -> Result<Option<Expression>> {
        match template {
            Some(_) => Ok(Some(Compiler::compile_template(template)?)),
            None => Ok(None),
        }
    }

    /// Compile a template string into a prepared expression
    pub fn compile_template(template: Option<&str>) -> Result<Expression> {
        let tmpl = req!(template);
        Ok(nodes::parse_template(tmpl)?)
    }
}
