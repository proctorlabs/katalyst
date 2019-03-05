mod env_provider;
mod header_provider;
mod http_provider;
mod regex_provider;

use crate::config::Gateway;
use crate::pipeline::PipelineState;
use crate::templates::KatalystTemplatePlaceholder;

pub use env_provider::EnvTemplateProvider;
pub use header_provider::HeaderTemplateProvider;
pub use regex_provider::RegexTemplateProvider;
pub use http_provider::HttpTemplateProvider;

struct PrecomputedPlaceholder {
    result: String,
}

impl PrecomputedPlaceholder {
    fn make(precomputed_str: String) -> Box<KatalystTemplatePlaceholder> {
        Box::new(PrecomputedPlaceholder {
            result: precomputed_str,
        })
    }
}

impl KatalystTemplatePlaceholder for PrecomputedPlaceholder {
    fn get_value(&self, _state: &PipelineState, _config: &Gateway) -> String {
        self.result.to_string()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(PrecomputedPlaceholder {
            result: self.result.to_owned(),
        })
    }
}
