mod claim;
mod env;
mod header;
mod http;
mod regex;

use crate::expression::*;
use crate::prelude::*;

pub use self::claim::ClaimExpressionBuilder;
pub use self::env::EnvExpressionBuilder;
pub use self::header::HeaderExpressionBuilder;
pub use self::http::HttpExpressionBuilder;
pub use self::regex::RegexExpressionBuilder;

#[derive(Debug)]
struct PrecomputedExpression {
    result: String,
}

impl PrecomputedExpression {
    fn make(precomputed_str: String) -> Box<CompiledExpression> {
        Box::new(PrecomputedExpression {
            result: precomputed_str,
        })
    }
}

impl CompiledExpression for PrecomputedExpression {
    fn get_value(&self, _: &Context) -> String {
        self.result.to_string()
    }

    fn duplicate(&self) -> Box<CompiledExpression> {
        PrecomputedExpression {
            result: self.result.to_owned(),
        }
        .boxed()
    }
}
