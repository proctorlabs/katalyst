#![allow(dead_code)]
#![allow(clippy::eval_order_dependence)]
use super::*;
use crate::prelude::*;
use std::fmt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, token, Ident, LitBool, LitInt, LitStr, Result, Token};

pub enum DynamicNode {
    Method(MethodNode),
    Text(LitStr),
    Number(LitInt),
    Bool(LitBool),
}

pub struct MethodNode {
    pub ident: Ident,
    pub dot_token: Token![.],
    pub ident2: Ident,
    pub paren_token: token::Paren,
    pub args: Punctuated<DynamicNode, Token![,]>,
}

impl Parse for MethodNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(MethodNode {
            ident: input.parse()?,
            dot_token: input.parse()?,
            ident2: input.parse()?,
            paren_token: parenthesized!(content in input),
            args: content.parse_terminated(DynamicNode::parse)?,
        })
    }
}

impl fmt::Debug for MethodNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method: {}(", self.ident.to_string())?;
        for arg in self.args.iter() {
            arg.fmt(f)?;
            write!(f, ", ")?;
        }
        write!(f, ")")
    }
}

impl Parse for DynamicNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            input.parse().map(DynamicNode::Method)
        } else if lookahead.peek(LitStr) {
            input.parse().map(DynamicNode::Text)
        } else if lookahead.peek(LitInt) {
            input.parse().map(DynamicNode::Number)
        } else if lookahead.peek(LitBool) {
            input.parse().map(DynamicNode::Bool)
        } else {
            Err(lookahead.error())
        }
    }
}

impl fmt::Debug for DynamicNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DynamicNode::Method(method) => method.fmt(f),
            DynamicNode::Text(text) => write!(f, "Text: {}", text.value()),
            DynamicNode::Number(number) => write!(f, "Number: {}", number.value()),
            DynamicNode::Bool(cnd) => write!(f, "Bool: {}", cnd.value),
        }
    }
}

impl DynamicNode {
    pub fn build(raw: &str) -> std::result::Result<DynamicNode, KatalystError> {
        match syn::parse_str(raw) {
            Ok(res) => Ok(res),
            Err(_) => Err(KatalystError::ConfigParseError),
        }
    }

    pub fn compile(
        &self,
        directory: &BuilderDirectory,
    ) -> std::result::Result<Arc<CompiledExpression>, KatalystError> {
        match self {
            DynamicNode::Method(node) => {
                let method_name = node.ident.to_string();
                let builder = directory.get(&method_name.as_str());
                match builder {
                    Some(b) => {
                        let mut args: Vec<Arc<CompiledExpression>> = vec![];
                        for arg in node.args.iter() {
                            args.push(arg.compile(directory)?);
                        }
                        let method = node.ident2.to_string();
                        Ok(Arc::new(CompiledExpressionNode {
                            name: method_name.to_string(),
                            args: args.clone(),
                            render_fn: b.make_fn(&method, &args)?,
                            result: ExpressionResultType::Text,
                        }))
                    }
                    None => Err(KatalystError::ConfigParseError),
                }
            }
            DynamicNode::Text(text) => Ok(Arc::new(text.value())),
            DynamicNode::Number(number) => Ok(Arc::new(number.value())),
            DynamicNode::Bool(cnd) => Ok(Arc::new(cnd.value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple_expression() {
        let exp = " some.thing (\"string\", true) ";
        let node: DynamicNode = syn::parse_str(exp).unwrap();
        println!("{:?}", node);
    }

    #[test]
    fn compile_from_expression() {
        let exp = " http.matched (\"test\") ";
        let node: DynamicNode = syn::parse_str(exp).unwrap();
        let mut directory = BuilderDirectory::default();
        let builder = Box::new(HttpBinding {});
        directory.insert(builder.identifier(), builder);
        let result = node.compile(&directory).unwrap();
        println!("{:?}", result);
    }
}
