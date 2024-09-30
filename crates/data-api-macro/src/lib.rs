#![allow(unused_variables)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn data_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;

    let mut fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => panic!("`#[data_model]` can only be used with structs"),
    };


    let mut fields = match fields {
        Fields::Named(ref mut fields) => {
            // for  f in fields.named.iter_mut() {
            //     f.attrs.push(syn::parse_quote!(#[export]));
            // }
            fields.named.clone()
        },
        _ => panic!("`#[prefab_resource]` can only be used with structs with named fields"),
    };


    let expanded = quote! {
        #(#attrs)*
        #vis struct #name {
            #fields

            pub id: i64,
            pub created: i64,
            pub updated: i64
        }
    };

    TokenStream::from(expanded)
}