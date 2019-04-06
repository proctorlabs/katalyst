mod builder;
mod compiler;
mod traits;

use builder::*;
pub use compiler::Providers;
pub use traits::{KatalystTemplatePlaceholder, KatalystTemplateProvider, Templatizable};

pub type StringTemplate = Vec<Box<KatalystTemplatePlaceholder>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_template_match() {
        let provider = Providers::default();
        let result = provider.process_template("/testing/test/{{boo=>rai}}/boom");
        assert_eq!(result.len(), 3);
        // assert_eq!(result[0], ("boo".to_string(), "rai".to_string()));
    }

    #[test]
    fn positive_template_match_with_whitespace() {
        let provider = Providers::default();
        let result = provider.process_template("/testing/test/{{ boo   =>  rai     }}/boom");
        assert_eq!(result.len(), 3);
        //assert_eq!(result[0], ("boo".to_string(), "rai".to_string()));
    }

    #[test]
    fn positive_template_match_with_nested_template() {
        let provider = Providers::default();
        let result = provider.process_template("/testing/test/{{boo=>rai=>me}}/boom");
        assert_eq!(result.len(), 3);
        //assert_eq!(result[0], ("boo".to_string(), "rai=>me".to_string()));
    }

    #[test]
    fn negative_template_match_regular_url() {
        let provider = Providers::default();
        let result = provider.process_template("/testing/test/boom?query=value&something=else");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn negative_template_match_handlebars_but_bad_pattern() {
        let provider = Providers::default();
        let result = provider
            .process_template("/testing/{{test->shouldn'tmatch}}/boom?query=value&something=else");
        assert_eq!(result.len(), 1);
    }
}
