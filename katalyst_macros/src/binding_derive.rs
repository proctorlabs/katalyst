use crate::attr::*;
use proc_macro::TokenStream;
use quote::*;

type BindingTuple = (Option<Box<ToTokens>>, Option<Box<ToTokens>>);

#[derive(Default)]
struct BindingMetadata {
    object_name: Option<Box<ToTokens>>,
    bindings: Vec<BindingTuple>,
}

fn read_attributes(attrs: &[syn::Attribute]) -> BindingMetadata {
    let mut result = BindingMetadata::default();
    for attr in attrs
        .iter()
        .filter(|a| !a.path.segments.is_empty() && a.path.segments[0].ident == "expression")
    {
        let container: BindingAttrParens = syn::parse2(attr.tts.clone()).unwrap();
        let mut binding: BindingTuple = (None, None);
        let mut def_binding = String::default();
        for item in container.contents.into_iter() {
            match item.ident.to_string().as_str() {
                "bind" => {
                    let mut tokens = proc_macro2::TokenStream::default();
                    item.val.to_tokens(&mut tokens);
                    def_binding = tokens.to_string();
                    binding.0 = Some(Box::new(item.val));
                }
                "call_name" => binding.1 = Some(Box::new(item.val)),
                "name" => result.object_name = Some(Box::new(item.val)),
                _ => panic!("Unknown!"),
            }
        }
        if binding.0.is_some() {
            if binding.1.is_none() {
                binding.1 = Some(Box::new(def_binding));
            }
            result.bindings.push(binding);
        }
    }
    result
}

pub fn impl_derive_expression_binding(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let mut metadata: BindingMetadata = read_attributes(&ast.attrs);
    if metadata.object_name.is_none() {
        metadata.object_name = Some(Box::new(ident.to_string().to_ascii_lowercase()));
    }

    let mut match_options = vec![];
    for binding in metadata.bindings.iter() {
        let check = &binding.1;
        let method = &binding.0;
        match_options.push(quote! {
            #check => {
                Ok(std::sync::Arc::new(#ident::#method))
            },
        });
    }
    let name = &metadata.object_name;

    let gen = quote! {
        impl From<#ident> for Box<ExpressionBinding> {
            fn from(item: #ident) -> Self {
                Box::new(item)
            }
        }

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
