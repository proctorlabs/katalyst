mod compiled;
mod nodes;

use crate::expression::*;
use compiled::*;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

type BuilderDirectory = HashMap<&'static str, Box<ExpressionBuilder>>;

//const TEMPLATE_FINDER_STR: &str = r"\{\{([^}]*)}}"; // Matches {{ }} templates
const METHOD: &str = r"\s*([^}(=>)\s]+)\s*(?:=>)\s*([^}\s]*)\s*";
const TEMPLATE: &str = r"\{\{\s*([^}(=>)\s]+)\s*(?:=>)\s*([^}\s]*)\s*}}";

lazy_static! {
    static ref TEMPLATE_MATCHER: Regex = Regex::new(TEMPLATE).unwrap();
    static ref METHOD_MATCHER: Regex = Regex::new(METHOD).unwrap();
}

pub struct Compiler {
    builders: BuilderDirectory,
}

impl Compiler {
    pub fn get_from_template(&self, placeholder_text: String) -> Arc<CompiledExpression> {
        match TEMPLATE_MATCHER.captures(&placeholder_text) {
            Some(cap) => {
                let key = &cap[1];
                let val = &cap[2];
                match self.builders.get(key) {
                    Some(p) => p.build(val.to_string()),
                    None => Arc::new(placeholder_text),
                }
            }
            None => Arc::new(placeholder_text),
        }
    }

    pub fn get_from_method(&self, placeholder_text: String) -> Arc<CompiledExpression> {
        match METHOD_MATCHER.captures(&placeholder_text) {
            Some(cap) => {
                let key = &cap[1];
                let val = &cap[2];
                match self.builders.get(key) {
                    Some(p) => p.build(val.to_string()),
                    None => Arc::new(placeholder_text),
                }
            }
            None => Arc::new(placeholder_text),
        }
    }

    pub fn register(&mut self, provider: Box<ExpressionBuilder>) {
        self.builders.insert(provider.identifier(), provider);
    }

    pub fn empty() -> Self {
        Compiler {
            builders: HashMap::new(),
        }
    }

    pub fn process_template_map(
        &self,
        template: &Option<HashMap<String, String>>,
    ) -> Option<HashMap<String, Expression>> {
        match template {
            Some(m) => Some(
                m.iter()
                    .map(|(k, v)| (k.to_string(), self.process_template(&v)))
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn process_template_option(&self, template: &Option<String>) -> Option<Expression> {
        match template {
            Some(s) => Some(self.process_template(&s)),
            None => None,
        }
    }

    pub fn process_template(&self, template: &str) -> Expression {
        let mut result_placeholders: Expression = vec![];
        if TEMPLATE_MATCHER.is_match(template) {
            let mut last_segment_index = 0;
            for cap in TEMPLATE_MATCHER.find_iter(template) {
                if cap.start() > last_segment_index {
                    let offset = cap.start() - last_segment_index;
                    let segment: String = template
                        .chars()
                        .skip(last_segment_index)
                        .take(offset)
                        .collect();
                    result_placeholders.push(Arc::new(segment));
                }
                result_placeholders.push(self.get_from_template(cap.as_str().to_owned()));
                last_segment_index = cap.end();
            }
            if last_segment_index < template.len() {
                let offset = template.len() - last_segment_index;
                let segment: String = template
                    .chars()
                    .skip(last_segment_index)
                    .take(offset)
                    .collect();
                result_placeholders.push(Arc::new(segment));
            }
        } else {
            result_placeholders.push(Arc::new(template.to_owned()));
        }
        result_placeholders
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
