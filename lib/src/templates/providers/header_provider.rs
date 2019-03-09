use crate::pipeline::PipelineState;
use crate::state::KatalystState;
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

#[derive(Debug)]
struct HeaderTemplatePlaceholder {
    header: String,
}

impl KatalystTemplatePlaceholder for HeaderTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &KatalystState) -> String {
        match &state.upstream.request {
            Some(s) => match s.headers().get(&self.header) {
                Some(t) => t.to_str().unwrap().to_string(),
                None => self.none().to_string(),
            },
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
