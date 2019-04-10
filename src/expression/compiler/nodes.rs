#![allow(dead_code)]
#![allow(clippy::eval_order_dependence)]
use std::fmt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, token, Ident, LitInt, LitStr, Result, Token};

pub enum DynamicNode {
    Method(MethodNode),
    Text(LitStr),
    Number(LitInt),
}

pub struct MethodNode {
    ident: Ident,
    paren_token: token::Paren,
    args: Punctuated<DynamicNode, Token![,]>,
}

impl Parse for MethodNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(MethodNode {
            ident: input.parse()?,
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
        }
    }
}
