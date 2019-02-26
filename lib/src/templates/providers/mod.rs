mod env;

use crate::config::Gateway;
use crate::pipeline::PipelineState;
use crate::templates::KatalystTemplatePlaceholder;

pub use env::EnvTemplateProvider;

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
    fn get_value(&self, _state: &PipelineState, _config: &Gateway) -> &str {
        &self.result
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(PrecomputedPlaceholder {
            result: self.result.to_owned(),
        })
    }
}
