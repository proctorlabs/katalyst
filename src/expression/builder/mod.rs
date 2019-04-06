mod claim_provider;
mod env_provider;
mod header_provider;
mod http_provider;
mod regex_provider;

use crate::expression::*;
use crate::prelude::*;

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
    fn get_value(&self, _: &Context) -> String {
        self.result.to_string()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        PrecomputedPlaceholder {
            result: self.result.to_owned(),
        }
        .boxed()
    }
}
