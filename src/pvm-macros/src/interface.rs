//! Interface macro implementation
//!
//! The `#[pvm::interface]` macro transforms a trait definition into a contract
//! reference struct that can be used to make type-safe cross-contract calls.

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{parse2, ItemTrait, TraitItem, FnArg, ReturnType, Pat};

use crate::compute_selector;

pub fn expand(input: TokenStream) -> TokenStream {
    let trait_def: ItemTrait = match parse2(input) {
        Ok(t) => t,
        Err(e) => return e.to_compile_error(),
    };

    let trait_name = &trait_def.ident;
    let trait_vis = &trait_def.vis;
    let ref_struct_name = format_ident!("{}Ref", trait_name);

    // Collect methods from the trait
    let mut method_impls = Vec::new();
    let mut selector_consts = Vec::new();

    for item in &trait_def.items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let method_name_str = method_name.to_string();

            // Compute selector
            let selector = compute_selector(&method_name_str);
            let selector_const = format_ident!("__SELECTOR_{}", method_name_str.to_uppercase());
            let sel_bytes = selector.iter().copied();

            selector_consts.push(quote! {
                const #selector_const: [u8; 4] = [#(#sel_bytes),*];
            });

            // Parse method signature
            let (arg_names, arg_types) = parse_method_args(&method.sig.inputs);
            let output = &method.sig.output;

            // Generate method implementation
            let method_impl = generate_method_impl(
                method_name,
                &selector_const,
                &arg_names,
                &arg_types,
                output,
            );

            method_impls.push(method_impl);
        }
    }

    // Generate the output
    quote! {
        // Keep the original trait for documentation/reference
        #trait_def

        /// Contract reference for calling #trait_name methods
        #[derive(Clone, Copy)]
        #trait_vis struct #ref_struct_name {
            address: pvm::Address,
        }

        impl #ref_struct_name {
            /// Create a new contract reference
            pub fn new(address: pvm::Address) -> Self {
                Self { address }
            }

            /// Get the contract address
            pub fn address(&self) -> pvm::Address {
                self.address
            }

            #(#selector_consts)*

            #(#method_impls)*
        }
    }
}

/// Parse method arguments, extracting names and types
fn parse_method_args(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
) -> (Vec<syn::Ident>, Vec<syn::Type>) {
    let mut names = Vec::new();
    let mut types = Vec::new();

    for arg in inputs {
        // Skip self parameters
        if let FnArg::Typed(pat_type) = arg {
            // Extract the argument name
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                names.push(pat_ident.ident.clone());
                types.push((*pat_type.ty).clone());
            }
        }
    }

    (names, types)
}

/// Generate the implementation for a single method
fn generate_method_impl(
    method_name: &syn::Ident,
    selector_const: &syn::Ident,
    arg_names: &[syn::Ident],
    arg_types: &[syn::Type],
    output: &ReturnType,
) -> TokenStream {
    // Build the method signature with &self
    let args_with_types: Vec<_> = arg_names
        .iter()
        .zip(arg_types.iter())
        .map(|(name, ty)| quote! { #name: #ty })
        .collect();

    // Build the encoding logic for arguments
    let encode_args = if arg_names.is_empty() {
        quote! { let args: &[u8] = &[]; }
    } else if arg_names.len() == 1 {
        // Single argument - encode directly
        let arg = &arg_names[0];
        quote! {
            let args = pvm::Encode::encode(&#arg);
        }
    } else {
        // Multiple arguments - encode as tuple
        let args = arg_names;
        quote! {
            let args = pvm::Encode::encode(&(#(#args.clone()),*));
        }
    };

    match output {
        ReturnType::Default => {
            // No return value
            quote! {
                pub fn #method_name(&self, #(#args_with_types),*) -> Result<(), pvm::ContractCallError> {
                    #encode_args
                    pvm::contract_call(&self.address, Self::#selector_const, &args)
                }
            }
        }
        ReturnType::Type(_, ret_type) => {
            // Has return value
            quote! {
                pub fn #method_name(&self, #(#args_with_types),*) -> Result<#ret_type, pvm::ContractCallError> {
                    #encode_args
                    pvm::call_and_decode(&self.address, Self::#selector_const, &args)
                }
            }
        }
    }
}
