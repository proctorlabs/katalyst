mod providers;
mod template_traits;

use providers::*;
use regex::Regex;
use std::collections::HashMap;
pub use template_traits::KatalystTemplatePlaceholder;
pub use template_traits::KatalystTemplateProvider;

const METHOD: &str = r"\s*([^}(=>)\s]+)\s*(?:=>)\s*([^}\s]*)\s*";
const TEMPLATE: &str = r"\{\{\s*([^}(=>)\s]+)\s*(?:=>)\s*([^}\s]*)\s*}}";

lazy_static! {
    static ref TEMPLATE_MATCHER: Regex = Regex::new(TEMPLATE).unwrap();
    static ref METHOD_MATCHER: Regex = Regex::new(METHOD).unwrap();
}

pub struct Providers {
    providers: HashMap<&'static str, Box<KatalystTemplateProvider>>,
}

impl Providers {
    pub fn get_from_template(&self, placeholder_text: String) -> Box<KatalystTemplatePlaceholder> {
        match TEMPLATE_MATCHER.captures(&placeholder_text) {
            Some(cap) => {
                let key = &cap[1];
                let val = &cap[2];
                match self.providers.get(key) {
                    Some(p) => p.build_placeholder(val.to_string()),
                    None => Box::new(placeholder_text),
                }
            }
            None => Box::new(placeholder_text),
        }
    }

    pub fn get_from_method(&self, placeholder_text: String) -> Box<KatalystTemplatePlaceholder> {
        match METHOD_MATCHER.captures(&placeholder_text) {
            Some(cap) => {
                let key = &cap[1];
                let val = &cap[2];
                match self.providers.get(key) {
                    Some(p) => p.build_placeholder(val.to_string()),
                    None => Box::new(placeholder_text),
                }
            }
            None => Box::new(placeholder_text),
        }
    }

    pub fn register(&mut self, provider: Box<KatalystTemplateProvider>) {
        self.providers.insert(provider.identifier(), provider);
    }

    pub fn empty() -> Self {
        Providers {
            providers: HashMap::new(),
        }
    }

    pub fn process_template(&self, template: &str) -> Vec<Box<KatalystTemplatePlaceholder>> {
        let mut result_placeholders: Vec<Box<KatalystTemplatePlaceholder>> = vec![];
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
                    result_placeholders.push(Box::new(segment));
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
                result_placeholders.push(Box::new(segment));
            }
        } else {
            result_placeholders.push(Box::new(template.to_owned()));
        }
        result_placeholders
    }
}

impl Default for Providers {
    fn default() -> Self {
        let mut providers = Providers::empty();
        providers.register(Box::new(EnvTemplateProvider {}));
        providers.register(Box::new(RegexTemplateProvider {}));
        providers.register(Box::new(HeaderTemplateProvider {}));
        providers.register(Box::new(HttpTemplateProvider {}));
        providers.register(Box::new(ClaimTemplateProvider {}));
        providers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_template_match() {
        let provider = Providers::default();
        let result = provider.process_template("/testing/test/{{boo=>rai}}/boom");
        assert_eq!(result.len(), 3);
        // assert_eq!(result[0], ("boo".to_string(), "rai".to_string()));
    }

    #[test]
    fn positive_template_match_with_whitespace() {
        let provider = Providers::default();
        let result = provider.process_template("/testing/test/{{ boo   =>  rai     }}/boom");
        assert_eq!(result.len(), 3);
        //assert_eq!(result[0], ("boo".to_string(), "rai".to_string()));
    }

    #[test]
    fn positive_template_match_with_nested_template() {
        let provider = Providers::default();
        let result = provider.process_template("/testing/test/{{boo=>rai=>me}}/boom");
        assert_eq!(result.len(), 3);
        //assert_eq!(result[0], ("boo".to_string(), "rai=>me".to_string()));
    }

    #[test]
    fn negative_template_match_regular_url() {
        let provider = Providers::default();
        let result = provider.process_template("/testing/test/boom?query=value&something=else");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn negative_template_match_handlebars_but_bad_pattern() {
        let provider = Providers::default();
        let result = provider
            .process_template("/testing/{{test->shouldn'tmatch}}/boom?query=value&something=else");
        assert_eq!(result.len(), 1);
    }
}
