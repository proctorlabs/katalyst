mod compiled;
pub(crate) mod nodes;

use crate::expression::*;
use compiled::*;
use std::{collections::HashMap, sync::Arc};

type BuilderDirectory = HashMap<&'static str, Box<ExpressionBinding>>;

pub struct Compiler {
    builders: BuilderDirectory,
}

impl Compiler {
    pub fn register(&mut self, provider: Box<ExpressionBinding>) {
        self.builders.insert(provider.identifier(), provider);
    }

    pub fn empty() -> Self {
        Compiler { builders: HashMap::new() }
    }

    pub fn compile_template_map(
        &self,
        template: &Option<HashMap<String, String>>,
    ) -> Result<Option<HashMap<String, Expression>>> {
        match template {
            Some(m) => Ok(Some({
                let mut result = HashMap::<String, Expression>::new();
                for i in m {
                    result.insert(i.0.to_string(), self.compile_template(Some(i.1))?);
                }
                result
            })),
            None => Ok(None),
        }
    }

    pub fn compile_template_option(&self, template: Option<&str>) -> Result<Option<Expression>> {
        match template {
            Some(_) => Ok(Some(self.compile_template(template)?)),
            None => Ok(None),
        }
    }

    pub fn compile_template(&self, raw_str: Option<&str>) -> Result<Expression> {
        if let Some(raw) = raw_str {
            Ok(nodes::parse_template(raw, &self.builders)?)
        } else {
            Err(GatewayError::ElementExpected("template"))
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        let mut providers = Compiler::empty();
        providers.register(Sys.into());
        providers.register(Http.into());
        providers.register(Auth.into());
        providers.register(Url.into());
        providers.register(Content.into());
        providers.register(Encode.into());
        providers.register(Decode.into());
        providers
    }
}
