use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

struct BindingAttrValues {
    pub ident: syn::Ident,
    pub equal: Token![=],
    pub val: syn::Expr,
}

impl Parse for BindingAttrValues {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(BindingAttrValues {
            ident: input.parse()?,
            equal: input.parse()?,
            val: input.parse()?,
        })
    }
}

struct BindingAttrParens {
    pub parens: syn::token::Paren,
    pub contents: Punctuated<BindingAttrValues, Token![,]>,
}

impl Parse for BindingAttrParens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let parens = parenthesized!(content in input);
        let contents = content.parse_terminated(BindingAttrValues::parse)?;
        Ok(BindingAttrParens { parens, contents })
    }
}

struct VariantMetadata {
    ident: String,
    method_expr: syn::Expr,
}

fn get_variant_metadata(variant: &syn::Variant) -> VariantMetadata {
    let ident = variant.ident.to_string().to_ascii_lowercase();
    let mut method_expr = syn::parse_str("default()").unwrap();
    for attr in variant
        .attrs
        .iter()
        .filter(|a| !a.path.segments.is_empty() && a.path.segments[0].ident == "expression")
    {
        let container: BindingAttrParens = syn::parse2(attr.tts.clone()).unwrap();
        for item in container.contents.iter() {
            match item.ident.to_string().as_str() {
                "method" => method_expr = item.val.clone(),
                _ => panic!("Unknown!"),
            }
        }
    }
    VariantMetadata { ident, method_expr }
}

//TODO: Finish this
pub fn impl_derive_expression_binding(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let name = ident.to_string().to_ascii_lowercase();
    let mut variants = vec![];

    match &ast.data {
        syn::Data::Enum(ref data) => {
            for variant in data.variants.iter() {
                variants.push(get_variant_metadata(variant));
            }
        }
        _ => panic!("ExpressionBinding only valid for Enum"),
    };

    let mut match_options = vec![];

    for variant in variants.iter() {
        let check = &variant.ident;
        let method = &variant.method_expr;
        match_options.push(quote! {
            #check => {
                Ok(std::sync::Arc::new(#ident::#method))
            },
        });
    }

    let gen = quote! {
        impl ExpressionBinding for #ident {
            fn identifier(&self) -> &'static str {
                #name
            }

            fn make_fn(&self, name: &str, args: &[ExpressionArg]) -> Result<ExpressionRenderMethod> {
                match name {
                    #(#match_options)*
                    _ => Err(GatewayError::ExpressionItemNotFound(name.to_string()))
                }
            }
        }
    };

    gen.into()
}
