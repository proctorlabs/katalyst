mod builder;
mod compiler;
mod traits;

use builder::*;
pub use compiler::Compiler;
use std::sync::Arc;
pub use traits::ExpressionRenderFn;
pub use traits::*;

pub type Expression = Vec<Arc<CompiledExpression>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_simple_expression() {
        let cmp = Compiler::default();
        let result = cmp
            .compile("blah blah {{ regex () }} blah blah {{ regex(xx) }} blah")
            .unwrap();
        println!("{:?}", result);
    }

    #[test]
    fn positive_template_match() {
        let provider = Compiler::default();
        let result = provider.process_template("/testing/test/{{boo=>rai}}/boom");
        assert_eq!(result.len(), 3);
        // assert_eq!(result[0], ("boo".to_string(), "rai".to_string()));
    }

    #[test]
    fn positive_template_match_with_whitespace() {
        let provider = Compiler::default();
        let result = provider.process_template("/testing/test/{{ boo   =>  rai     }}/boom");
        assert_eq!(result.len(), 3);
        //assert_eq!(result[0], ("boo".to_string(), "rai".to_string()));
    }

    #[test]
    fn positive_template_match_with_nested_template() {
        let provider = Compiler::default();
        let result = provider.process_template("/testing/test/{{boo=>rai=>me}}/boom");
        assert_eq!(result.len(), 3);
        //assert_eq!(result[0], ("boo".to_string(), "rai=>me".to_string()));
    }

    #[test]
    fn negative_template_match_regular_url() {
        let provider = Compiler::default();
        let result = provider.process_template("/testing/test/boom?query=value&something=else");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn negative_template_match_handlebars_but_bad_pattern() {
        let provider = Compiler::default();
        let result = provider
            .process_template("/testing/{{test->shouldn'tmatch}}/boom?query=value&something=else");
        assert_eq!(result.len(), 1);
    }
}
