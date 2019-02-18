mod template_traits;

use regex::Regex;

pub use template_traits::KatalystTemplatePlaceholder;
pub use template_traits::KatalystTemplateProvider;

lazy_static! {
    static ref TEMPLATE_PATTERN: Regex =
        Regex::new(r"\{\{\s*([^}(=>)\s]+)\s*(?:=>)\s*([^}\s]*)\s*}}").unwrap();
}

fn find_matches(template: &str) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = vec![];
    if TEMPLATE_PATTERN.is_match(template) {
        for cap in TEMPLATE_PATTERN.captures_iter(template) {
            let cap_group = ((&cap[1]).to_string(), (&cap[2]).to_string());
            result.push(cap_group);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_template_match() {
        let result = find_matches("/testing/test/{{boo=>rai}}/boom");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], ("boo".to_string(), "rai".to_string()));
    }

    #[test]
    fn positive_template_match_with_whitespace() {
        let result = find_matches("/testing/test/{{ boo   =>  rai     }}/boom");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], ("boo".to_string(), "rai".to_string()));
    }

    #[test]
    fn positive_template_match_with_nested_template() {
        let result = find_matches("/testing/test/{{boo=>rai=>me}}/boom");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], ("boo".to_string(), "rai=>me".to_string()));
    }

    #[test]
    fn negative_template_match_regular_url() {
        let result = find_matches("/testing/test/boom?query=value&something=else");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn negative_template_match_handlebars_but_bad_pattern() {
        let result =
            find_matches("/testing/{{test->shouldn'tmatch}}/boom?query=value&something=else");
        assert_eq!(result.len(), 0);
    }
}
