//! Event macro implementation

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemStruct};

use crate::compute_selector;

pub fn expand(input: TokenStream) -> TokenStream {
    let item: ItemStruct = match parse2(input) {
        Ok(item) => item,
        Err(e) => return e.to_compile_error(),
    };

    let name = &item.ident;
    let vis = &item.vis;
    let attrs = &item.attrs;
    let fields = &item.fields;
    let name_str = name.to_string();

    // Compute event signature hash (first topic)
    let sig_hash = compute_selector(&name_str);
    let mut topic_bytes = [0u8; 32];
    topic_bytes[0..4].copy_from_slice(&sig_hash);
    let topic_tokens: Vec<_> = topic_bytes.iter().map(|b| quote! { #b }).collect();

    quote! {
        #(#attrs)*
        #[derive(pvm::Encode)]
        #vis struct #name #fields

        impl pvm::Event for #name {
            fn topics(&self) -> alloc::vec::Vec<[u8; 32]> {
                // Return the event signature as the only topic
                alloc::vec![[#(#topic_tokens),*]]
            }
        }
    }
}
