//! Call macro implementation

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{parse2, ItemFn, FnArg, Type, ReturnType};

use crate::compute_selector;

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

    let name_str = name.to_string();
    let selector = compute_selector(&name_str);
    let selector_const = format_ident!("__PVM_SELECTOR_{}", name_str.to_uppercase());
    let wrapper_fn = format_ident!("__pvm_call_{}", name);

    // Parse the first argument to determine if it takes state
    let (state_type, is_mutable) = parse_state_arg(inputs);

    // Generate the wrapper function
    let wrapper = if let Some(state_ty) = state_type {
        if is_mutable {
            // Mutable state: load, call, save
            generate_mutable_wrapper(&wrapper_fn, name, &state_ty, output)
        } else {
            // Immutable state: load, call (no save)
            generate_immutable_wrapper(&wrapper_fn, name, &state_ty, output)
        }
    } else {
        // No state argument
        generate_stateless_wrapper(&wrapper_fn, name, output)
    };

    let sel_bytes = selector.iter();

    quote! {
        #(#attrs)*
        #vis fn #name(#inputs) #output #block

        /// Selector for this function
        pub const #selector_const: [u8; 4] = [#(#sel_bytes),*];

        #wrapper
    }
}

fn parse_state_arg(inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>) -> (Option<Type>, bool) {
    if let Some(FnArg::Typed(pat_type)) = inputs.first() {
        // Check if it's a reference type
        if let Type::Reference(ref_type) = &*pat_type.ty {
            let is_mutable = ref_type.mutability.is_some();
            return (Some((*ref_type.elem).clone()), is_mutable);
        }
    }
    (None, false)
}

fn generate_mutable_wrapper(
    wrapper_fn: &syn::Ident,
    original_fn: &syn::Ident,
    state_ty: &Type,
    output: &ReturnType,
) -> TokenStream {
    let call_and_return = match output {
        ReturnType::Default => quote! {
            #original_fn(&mut state);
            state.save();
            pvm::return_value(&[]);
        },
        ReturnType::Type(_, _) => quote! {
            let result = #original_fn(&mut state);
            state.save();
            let encoded = pvm::Encode::encode(&result);
            pvm::return_value(&encoded);
        },
    };

    quote! {
        #[doc(hidden)]
        pub fn #wrapper_fn() {
            let mut state = <#state_ty>::load().unwrap_or_default();
            #call_and_return
        }
    }
}

fn generate_immutable_wrapper(
    wrapper_fn: &syn::Ident,
    original_fn: &syn::Ident,
    state_ty: &Type,
    output: &ReturnType,
) -> TokenStream {
    let call_and_return = match output {
        ReturnType::Default => quote! {
            #original_fn(&state);
            pvm::return_value(&[]);
        },
        ReturnType::Type(_, _) => quote! {
            let result = #original_fn(&state);
            let encoded = pvm::Encode::encode(&result);
            pvm::return_value(&encoded);
        },
    };

    quote! {
        #[doc(hidden)]
        pub fn #wrapper_fn() {
            let state = <#state_ty>::load().unwrap_or_default();
            #call_and_return
        }
    }
}

fn generate_stateless_wrapper(
    wrapper_fn: &syn::Ident,
    original_fn: &syn::Ident,
    output: &ReturnType,
) -> TokenStream {
    let call_and_return = match output {
        ReturnType::Default => quote! {
            #original_fn();
            pvm::return_value(&[]);
        },
        ReturnType::Type(_, _) => quote! {
            let result = #original_fn();
            let encoded = pvm::Encode::encode(&result);
            pvm::return_value(&encoded);
        },
    };

    quote! {
        #[doc(hidden)]
        pub fn #wrapper_fn() {
            #call_and_return
        }
    }
}
