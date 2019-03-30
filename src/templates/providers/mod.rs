mod claim_provider;
mod env_provider;
mod header_provider;
mod http_provider;
mod regex_provider;

use crate::pipeline::PipelineState;
use crate::state::KatalystState;
use crate::templates::KatalystTemplatePlaceholder;

pub use claim_provider::ClaimTemplateProvider;
pub use env_provider::EnvTemplateProvider;
pub use header_provider::HeaderTemplateProvider;
pub use http_provider::HttpTemplateProvider;
pub use regex_provider::RegexTemplateProvider;

#[derive(Debug)]
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
    fn get_value(&self, _state: &PipelineState, _config: &KatalystState) -> String {
        self.result.to_string()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        PrecomputedPlaceholder {
            result: self.result.to_owned(),
        }
        .boxed()
    }
}
