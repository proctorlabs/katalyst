use crate::config::Gateway;
use crate::pipeline::PipelineState;
use crate::templates::{KatalystTemplatePlaceholder, KatalystTemplateProvider};

pub struct EnvTemplateProvider {}

pub struct EnvTemplatePlaceholder {
    result: String,
}

impl KatalystTemplateProvider for EnvTemplateProvider {
    fn identifier(&self) -> String {
        "env".to_string()
    }
    
    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder> {
        Box::new(EnvTemplatePlaceholder{
            result: std::env::var_os(value).expect("Environment variable not set!").to_str().unwrap().to_owned()
        })
    }
}

impl KatalystTemplatePlaceholder for EnvTemplatePlaceholder {
    fn get_value(&self, _state: &PipelineState, _config: &Gateway) -> String {
        self.result.to_owned()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(EnvTemplatePlaceholder{
            result: self.result.to_owned()
        })
    }
}