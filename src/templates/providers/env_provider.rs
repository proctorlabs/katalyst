use super::PrecomputedPlaceholder;
use crate::templates::{KatalystTemplatePlaceholder, KatalystTemplateProvider};

pub struct EnvTemplateProvider {}

impl KatalystTemplateProvider for EnvTemplateProvider {
    fn identifier(&self) -> &'static str {
        "env"
    }

    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder> {
        PrecomputedPlaceholder::make(
            std::env::var_os(value)
                .expect("Environment variable not set!")
                .to_str()
                .unwrap_or_default()
                .to_owned(),
        )
    }
}
