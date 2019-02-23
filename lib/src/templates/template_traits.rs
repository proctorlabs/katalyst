use crate::config::Gateway;
use crate::pipeline::PipelineState;

/// This is the trait used by Katalyst for building the placeholders used in a downstream URL template
pub trait KatalystTemplateProvider {
    /// The identifier in this template to locate that this provider should be used
    fn identifier(&self) -> String;
    /// Construct a new KatalystTemplatePlaceholder to use in this config.
    /// Note that these are reused and should be immutable once made.
    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder>;
}

/// This provides the actual value replacement used in the downstream URL template
pub trait KatalystTemplatePlaceholder: Send {
    /// Returns the string value that should be used as a replacement for this Placeholder in the pipeline context
    fn get_value(&self, state: &PipelineState, config: &Gateway) -> String;

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder>;
}

impl KatalystTemplatePlaceholder for String {
    fn get_value(&self, _state: &PipelineState, _config: &Gateway) -> String {
        self.to_string()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(self.to_owned())
    }
}
