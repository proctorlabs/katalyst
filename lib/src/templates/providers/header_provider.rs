use crate::config::Gateway;
use crate::pipeline::PipelineState;
use crate::templates::{KatalystTemplatePlaceholder, KatalystTemplateProvider};

pub struct HeaderTemplateProvider {}

impl KatalystTemplateProvider for HeaderTemplateProvider {
    fn identifier(&self) -> &'static str {
        "header"
    }

    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder> {
        Box::new(HeaderTemplatePlaceholder { header: value })
    }
}

struct HeaderTemplatePlaceholder {
    header: String,
}

impl KatalystTemplatePlaceholder for HeaderTemplatePlaceholder {
    fn get_value(&self, _state: &PipelineState, _config: &Gateway) -> String {
        match _state.upstream_request.headers().get(&self.header) {
            Some(s) => s.to_str().unwrap().to_string(),
            None => self.none().to_string(),
        }
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        HeaderTemplatePlaceholder {
            header: self.header.to_owned(),
        }
        .boxed()
    }
}
