use crate::pipeline::PipelineState;
use crate::templates::{KatalystTemplatePlaceholder, KatalystTemplateProvider};

pub struct ClaimTemplateProvider {}

impl KatalystTemplateProvider for ClaimTemplateProvider {
    fn identifier(&self) -> &'static str {
        "claim"
    }

    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder> {
        Box::new(ClaimTemplatePlaceholder { claim_key: value })
    }
}

#[derive(Debug)]
struct ClaimTemplatePlaceholder {
    claim_key: String,
}

impl KatalystTemplatePlaceholder for ClaimTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState) -> String {
        if let Some(auth_info) = &state.context.authentication {
            auth_info.get_claim(self.claim_key.to_string())
        } else {
            self.none().to_string()
        }
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        ClaimTemplatePlaceholder {
            claim_key: self.claim_key.to_owned(),
        }
        .boxed()
    }
}
