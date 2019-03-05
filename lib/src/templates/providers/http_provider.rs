use crate::config::Gateway;
use crate::pipeline::PipelineState;
use crate::templates::{KatalystTemplatePlaceholder, KatalystTemplateProvider};

pub struct HttpTemplateProvider {}

impl KatalystTemplateProvider for HttpTemplateProvider {
    fn identifier(&self) -> &'static str {
        "http"
    }

    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder> {
        match value.as_str() {
            "method" => HttpMethodTemplatePlaceholder {}.duplicate(),
            "ip" => HttpIPTemplatePlaceholder {}.duplicate(),
            "path" => HttpUriTemplatePlaceholder {}.duplicate(),
            &_ => HttpMethodTemplatePlaceholder {}.duplicate(),
        }
    }
}

struct HttpMethodTemplatePlaceholder {}
struct HttpIPTemplatePlaceholder {}
struct HttpUriTemplatePlaceholder {}
struct HttpQueryTemplatePlaceholder {}

impl KatalystTemplatePlaceholder for HttpMethodTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &Gateway) -> String {
        state.upstream_request.method().as_str().to_owned()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(HttpMethodTemplatePlaceholder {})
    }
}

impl KatalystTemplatePlaceholder for HttpIPTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &Gateway) -> String {
        state.remote_addr.ip().to_string()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(HttpIPTemplatePlaceholder {})
    }
}

impl KatalystTemplatePlaceholder for HttpUriTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &Gateway) -> String {
        state.upstream_request.uri().path().to_string()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(HttpUriTemplatePlaceholder {})
    }
}

impl KatalystTemplatePlaceholder for HttpQueryTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &Gateway) -> String {
        state
            .upstream_request
            .uri()
            .query()
            .unwrap_or_default()
            .to_string()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(HttpQueryTemplatePlaceholder {})
    }
}
