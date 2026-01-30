//! Init macro implementation

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{parse2, ItemFn};

pub fn expand(input: TokenStream) -> TokenStream {
    let func: ItemFn = match parse2(input) {
        Ok(f) => f,
        Err(e) => return e.to_compile_error(),
    };

    let name = &func.sig.ident;
    let vis = &func.vis;
    let attrs = &func.attrs;
    let block = &func.block;
    let inputs = &func.sig.inputs;
    let output = &func.sig.output;

    let wrapper_fn = format_ident!("__pvm_init_{}", name);

    // For now, assume constructor takes no arguments (we'll add arg parsing later)
    // and returns the storage type
    quote! {
        #(#attrs)*
        #vis fn #name(#inputs) #output #block

        #[doc(hidden)]
        pub fn #wrapper_fn() {
            // TODO: Parse constructor arguments from input
            let state = #name();
            state.save();
            pvm::return_value(&[]);
        }
    }
}
