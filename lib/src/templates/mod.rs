mod template_traits;
mod providers;

use regex::Regex;
use std::collections::HashMap;
pub use providers::*;
pub use template_traits::KatalystTemplatePlaceholder;
pub use template_traits::KatalystTemplateProvider;

lazy_static! {
    static ref TEMPLATE_PATTERN: Regex =
        Regex::new(r"\{\{\s*([^}(=>)\s]+)\s*(?:=>)\s*([^}\s]*)\s*}}").unwrap();
}

pub struct Providers {
    providers: HashMap<String, Box<KatalystTemplateProvider + Send>>,
}

impl Providers {
    pub fn build_placeholder(&self, placeholder_text: String) -> Box<KatalystTemplatePlaceholder> {
        for cap in TEMPLATE_PATTERN.captures(&placeholder_text)
        {
            let id = &cap[1];
            let val = &cap[2];
            let provider = self.providers.get(id);
            if provider.is_some() {
                for p in provider.iter() {
                    return p.build_placeholder(val.to_string());
                }
            }
        }
        Box::new(placeholder_text)

    }

    pub fn register(&mut self, provider: Box<KatalystTemplateProvider + Send>) {
        self.providers.insert(provider.identifier(), provider);
    }

    pub fn empty() -> Self {
        Providers {
            providers: HashMap::new(),
        }
    }

    pub fn process_template(&self, template: &str) -> Vec<Box<KatalystTemplatePlaceholder>> {
        let mut result_placeholders: Vec<Box<KatalystTemplatePlaceholder>> = vec![];
        if TEMPLATE_PATTERN.is_match(template) {
            let mut last_segment_index = 0;
            for cap in TEMPLATE_PATTERN.find_iter(template)
            {
                if cap.start() > last_segment_index {
                    let offset = cap.start() - last_segment_index;
                    let segment: String = template
                        .chars()
                        .skip(last_segment_index)
                        .take(offset)
                        .collect();
                    println!("{}", segment);
                    result_placeholders.push(Box::new(segment));
                }
                result_placeholders.push(self.build_placeholder(cap.as_str().to_owned()));
                println!("{}", cap.as_str());
                last_segment_index = cap.end();
            }
            if last_segment_index < template.len() {
                let offset = template.len() - last_segment_index;
                let segment: String = template
                    .chars()
                    .skip(last_segment_index)
                    .take(offset)
                    .collect();
                println!("{}", segment);
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
        let mut result_map: HashMap<String, Box<KatalystTemplateProvider + Send>> = HashMap::new();
        result_map.insert(EnvTemplateProvider{}.identifier(), Box::new(EnvTemplateProvider{}));
        Providers {
            providers: result_map,
        }
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
