use crate::config::Gateway;
use crate::pipeline::PipelineState;

/// This is the trait used by Katalyst for building the placeholders used in a downstream URL template
pub trait KatalystTemplateProvider: Send + Sync {
    /// The identifier in this template to locate that this provider should be used
    fn identifier(&self) -> &'static str;
    /// Construct a new KatalystTemplatePlaceholder to use in this config.
    /// Note that these are reused and should be immutable once made.
    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder>;
}

lazy_static! {
    static ref DEF_STRING: String = String::default();
}

/// This provides the actual value replacement used in the downstream URL template
pub trait KatalystTemplatePlaceholder: Sync + Send {
    /// Returns the string value that should be used as a replacement for this Placeholder in the pipeline context
    fn get_value(&self, state: &PipelineState, config: &Gateway) -> String;

    /// Creates a boxed duplicate of this placeholder
    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder>;

    /// Returned when no match is found for placeholder
    fn none(&self) -> &String {
        &DEF_STRING
    }
}

impl KatalystTemplatePlaceholder for String {
    fn get_value(&self, _state: &PipelineState, _config: &Gateway) -> String {
        self.to_string()
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        Box::new(self.to_owned())
    }
}
