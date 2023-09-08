use proc_macro::TokenStream;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::parse_quote;


#[proc_macro_derive(MyProto)]
pub fn derive_proto(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ret = derive_proto_impl(&input)
        .map(|x| x.to_token_stream())
        .expect("Error!!!!!!");
    ret.into()
}

fn derive_proto_impl(input: &syn::DeriveInput) -> Result<syn::ItemImpl> {
    if !matches!(input.data, syn::Data::Struct { .. }) {
        bail!("{:?} only structs can derive Proto", &input);
    }

    let ident = &input.ident;

    Ok(parse_quote! {
        impl zenoh_dissector::MyProtoTrait for #ident {
        }
    })
}
