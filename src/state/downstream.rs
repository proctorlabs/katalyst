use crate::templates::StringTemplate;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Downstream {
    pub host: String,
    pub path: StringTemplate,
    pub method: Option<String>,
    pub query: Option<HashMap<String, StringTemplate>>,
    pub headers: Option<HashMap<String, StringTemplate>>,
    pub body: Option<StringTemplate>,
}
