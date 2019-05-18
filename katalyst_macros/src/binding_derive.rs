use proc_macro::TokenStream;

//TODO: Finish this
pub fn impl_derive_expression_binding(_ast: &syn::DeriveInput) -> TokenStream {
    unimplemented!()
    // let name = &ast.ident;

    // match &ast.data {
    //     syn::Data::Enum(ref data) => {
    //         unimplemented!()
    //     },
    //     _ => panic!("ExpressionBinding only valid for Enum"),
    // };

    // let gen = quote! {
    //     impl ExpressionBinding for #name {
    //         fn identifier(&self) -> &'static str {
    //             #name
    //         }

    //         fn make_fn(&self, name: &str, args: &[ExpressionArg]) -> Result<ExpressionRenderMethod, GatewayError> {
    //             match name {
    //                 _ => Err(GatewayError::ExpressionItemNotFound(#name.to_string()))
    //             }
    //         }
    //     }
    // };

    // gen.into()
}
