#![recursion_limit = "128"]

extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;

pub(crate) mod attr;
mod binding_derive;

use proc_macro::TokenStream;

#[proc_macro_derive(ExpressionBinding, attributes(expression))]
pub fn expression_binding_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    binding_derive::impl_derive_expression_binding(&ast)
}
