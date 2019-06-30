use crate::{HashedPassword, HtpasswdError};
use pest::Parser;
use pest_derive::*;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "htpasswd.pest"]
#[allow(dead_code)]
struct HtpasswdParser;

/// Parse the provided string as the contents of an HTPASSWD file.
/// This returns a hashmap of usernames mapped to their password hashes.
pub fn parse(input: &str) -> Result<HashMap<String, HashedPassword>, HtpasswdError> {
    let mut result = HashMap::new();
    let tokens = HtpasswdParser::parse(Rule::entries, input)?;
    let mut username = String::default();
    for pair in tokens.into_iter() {
        match pair.as_rule() {
            Rule::user_name => username = pair.as_str().into(),
            Rule::bcrypt_pass => {
                result.insert(username, HashedPassword::Bcrypt(pair.as_str().into()));
                username = String::default();
            }
            Rule::sha_pass => {
                result.insert(username, HashedPassword::Sha(pair.as_str().into()));
                username = String::default();
            }
            Rule::md5_pass => {
                result.insert(username, HashedPassword::Md5(pair.as_str().into()));
                username = String::default();
            }
            Rule::unknown_pass => {
                result.insert(username, HashedPassword::Unknown(pair.as_str().into()));
                username = String::default();
            }
            _ => continue,
        }
    }
    Ok(result)
}

impl From<pest::error::Error<Rule>> for HtpasswdError {
    fn from(e: pest::error::Error<Rule>) -> HtpasswdError {
        HtpasswdError::ParseError(format!("{}", e))
    }
}
