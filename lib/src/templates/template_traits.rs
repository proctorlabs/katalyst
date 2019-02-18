use crate::config::Gateway;
use crate::pipeline::PipelineState;

/// This is the trait used by Katalyst for building the placeholders used in a downstream URL template
pub trait KatalystTemplateProvider<T>
where
    T: KatalystTemplatePlaceholder + Send,
{
    /// The identifier in this template to locate that this provider should be used
    fn identifier() -> &'static str;
    /// Construct a new KatalystTemplatePlaceholder to use in this config.
    /// Note that these are reused and should be immutable once made.
    fn build_placeholder(value: String) -> Box<T>;
}

/// This provides the actual value replacement used in the downstream URL template
pub trait KatalystTemplatePlaceholder {
    /// Returns the string value that should be used as a replacement for this Placeholder in the pipeline context
    fn get_value(state: &PipelineState, config: &Gateway) -> String;
}
