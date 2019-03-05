use crate::config::Gateway;
use crate::pipeline::PipelineState;
use crate::templates::{KatalystTemplatePlaceholder, KatalystTemplateProvider};
use std::str::FromStr;

pub struct RegexTemplateProvider {}

impl KatalystTemplateProvider for RegexTemplateProvider {
    fn identifier(&self) -> &'static str {
        "regex"
    }

    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder> {
        Box::new(RegexTemplatePlaceholder { val: value })
    }
}

struct RegexTemplatePlaceholder {
    val: String,
}

impl KatalystTemplatePlaceholder for RegexTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &Gateway) -> String {
        match &state.captures {
            Some(caps) => {
                let res = caps.get(&self.val).unwrap_or(self.none());
                String::from_str(res).unwrap().to_string()
            }
            None => self.none().to_string(),
        }
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(RegexTemplatePlaceholder {
            val: self.val.to_owned(),
        })
    }
}
