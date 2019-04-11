mod compiled;
mod nodes;

use crate::expression::*;
use compiled::*;
use nodes::DynamicNode;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

type BuilderDirectory = HashMap<&'static str, Box<ExpressionBuilder>>;

lazy_static! {
    static ref TEMPLATE_FINDER: Regex = Regex::new(r"\{{2}((?:[^}])*)}}").unwrap(); // Matches {{ }} templates
}

pub struct Compiler {
    builders: BuilderDirectory,
}

impl Compiler {
    pub fn register(&mut self, provider: Box<ExpressionBuilder>) {
        self.builders.insert(provider.identifier(), provider);
    }

    pub fn empty() -> Self {
        Compiler {
            builders: HashMap::new(),
        }
    }

    pub fn compile_template_map(
        &self,
        template: &Option<HashMap<String, String>>,
    ) -> Option<HashMap<String, Expression>> {
        match template {
            Some(m) => Some(
                m.iter()
                    .filter_map(|(k, v)| match self.compile_template_option(Some(v)) {
                        Some(x) => Some((k.to_string(), x)),
                        None => None,
                    })
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn compile_template_option(&self, template: Option<&str>) -> Option<Expression> {
        match self.compile_template(template) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }

    pub fn compile_template(&self, raw_str: Option<&str>) -> Result<Expression, KatalystError> {
        if let Some(raw) = raw_str {
            let mut results: Expression = vec![];
            let mut last_segment_index = 0;
            if TEMPLATE_FINDER.is_match(raw) {
                for cap in TEMPLATE_FINDER.captures_iter(raw) {
                    let (mtch, expr) = (cap.get(0).unwrap(), &cap[1]);
                    if mtch.start() > last_segment_index {
                        let offset = mtch.start() - last_segment_index;
                        let segment: String =
                            raw.chars().skip(last_segment_index).take(offset).collect();
                        results.push(Arc::new(segment));
                    }
                    let node = DynamicNode::build(expr)?;
                    results.push(node.compile(&self.builders)?);
                    last_segment_index = mtch.end();
                }
            }
            if last_segment_index == 0 || last_segment_index < raw.len() {
                let offset = raw.len() - last_segment_index;
                let segment: String = raw.chars().skip(last_segment_index).take(offset).collect();
                results.push(Arc::new(segment));
            }
            Ok(results)
        } else {
            Err(KatalystError::NotFound)
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        let mut providers = Compiler::empty();
        providers.register(Box::new(EnvExpressionBuilder {}));
        providers.register(Box::new(RegexExpressionBuilder {}));
        providers.register(Box::new(HeaderExpressionBuilder {}));
        providers.register(Box::new(HttpExpressionBuilder {}));
        providers.register(Box::new(ClaimExpressionBuilder {}));
        providers
    }
}
