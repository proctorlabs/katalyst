use crate::pipeline::PipelineState;
use crate::state::KatalystState;
use crate::templates::{KatalystTemplatePlaceholder, KatalystTemplateProvider};

pub struct HttpTemplateProvider {}

impl KatalystTemplateProvider for HttpTemplateProvider {
    fn identifier(&self) -> &'static str {
        "http"
    }

    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder> {
        match value.as_str() {
            "method" => HttpMethodTemplatePlaceholder {}.boxed(),
            "ip" => HttpIPTemplatePlaceholder {}.boxed(),
            "path" => HttpUriTemplatePlaceholder {}.boxed(),
            &_ => HttpMethodTemplatePlaceholder {}.boxed(),
        }
    }
}

#[derive(Debug)]
struct HttpMethodTemplatePlaceholder {}
#[derive(Debug)]
struct HttpIPTemplatePlaceholder {}
#[derive(Debug)]
struct HttpUriTemplatePlaceholder {}
#[derive(Debug)]
struct HttpQueryTemplatePlaceholder {}

impl KatalystTemplatePlaceholder for HttpMethodTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &KatalystState) -> String {
        match &state.upstream.request {
            Some(s) => s.method().as_str().to_owned(),
            None => String::default(),
        }
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        HttpMethodTemplatePlaceholder {}.boxed()
    }
}

impl KatalystTemplatePlaceholder for HttpIPTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &KatalystState) -> String {
        state.remote_addr.ip().to_string()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        HttpIPTemplatePlaceholder {}.boxed()
    }
}

impl KatalystTemplatePlaceholder for HttpUriTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &KatalystState) -> String {
        match &state.upstream.request {
            Some(s) => s.uri().path().to_string(),
            None => String::default(),
        }
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        HttpUriTemplatePlaceholder {}.boxed()
    }
}

impl KatalystTemplatePlaceholder for HttpQueryTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &KatalystState) -> String {
        match &state.upstream.request {
            Some(s) => s.uri().query().unwrap_or_default().to_string(),
            None => String::default(),
        }
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        HttpQueryTemplatePlaceholder {}.boxed()
    }
}
