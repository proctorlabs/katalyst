use crate::expression::*;
use crate::prelude::*;

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
    fn get_value(&self, ctx: &Context) -> String {
        if let Some(auth_info) = &ctx.context.authentication {
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
