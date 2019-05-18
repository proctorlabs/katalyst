#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;

mod binding;
mod binding_derive;

use proc_macro::TokenStream;

#[proc_macro]
pub fn binding(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    binding::binding_impl(ast)
}

#[proc_macro_derive(ExpressionBinding, attributes(method))]
pub fn expression_binding_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    binding_derive::impl_derive_expression_binding(&ast)
}
