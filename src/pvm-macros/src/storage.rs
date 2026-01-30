//! Storage macro implementation

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemStruct};

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

    // Generate the storage key as bytes
    let key_bytes: [u8; 32] = {
        let mut key = [0u8; 32];
        let name_bytes = name_str.as_bytes();
        for (i, &b) in name_bytes.iter().enumerate() {
            if i < 32 {
                key[i] = b;
            } else {
                key[i % 32] ^= b;
            }
        }
        key
    };

    let key_tokens: Vec<_> = key_bytes.iter().map(|b| quote! { #b }).collect();

    quote! {
        #(#attrs)*
        #[derive(pvm::Encode, pvm::Decode, Default)]
        #vis struct #name #fields

        impl #name {
            /// Storage key for this contract's state
            pub const STORAGE_KEY: [u8; 32] = [#(#key_tokens),*];

            /// Load the contract state from storage
            pub fn load() -> Option<Self> {
                pvm::get_storage(&Self::STORAGE_KEY)
            }

            /// Save the contract state to storage
            pub fn save(&self) {
                pvm::set_storage(&Self::STORAGE_KEY, self);
            }
        }
    }
}
