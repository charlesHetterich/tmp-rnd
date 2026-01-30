//! Contract macro implementation
//!
//! The `#[pvm::contract]` attribute transforms a module into a complete PVM contract,
//! generating all necessary boilerplate.

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{parse2, ItemMod, Item, ItemFn, ItemStruct, Attribute, FnArg, ReturnType, Pat};

use crate::compute_selector;

pub fn expand(input: TokenStream) -> TokenStream {
    let module: ItemMod = match parse2(input) {
        Ok(m) => m,
        Err(e) => return e.to_compile_error(),
    };

    let mod_name = &module.ident;
    let mod_vis = &module.vis;
    let mod_attrs = &module.attrs;

    // Get module content
    let content = match &module.content {
        Some((_, items)) => items.clone(),
        None => {
            return syn::Error::new_spanned(
                &module,
                "#[pvm::contract] requires a module with content (not just a declaration)"
            ).to_compile_error();
        }
    };

    // Collect information about the contract
    let mut storage_struct: Option<ItemStruct> = None;
    let mut init_fn: Option<ItemFn> = None;
    let mut call_fns: Vec<ItemFn> = Vec::new();
    let mut other_items: Vec<Item> = Vec::new();

    for item in content {
        match item {
            Item::Struct(s) if has_attr(&s.attrs, "storage") => {
                storage_struct = Some(remove_pvm_attr(s));
            }
            Item::Fn(f) if has_attr(&f.attrs, "init") => {
                init_fn = Some(remove_pvm_attr_fn(f));
            }
            Item::Fn(f) if has_attr(&f.attrs, "call") => {
                call_fns.push(remove_pvm_attr_fn(f));
            }
            _ => other_items.push(item),
        }
    }

    // Generate storage code
    let storage_code = storage_struct.as_ref().map(|s| {
        let name = &s.ident;
        let vis = &s.vis;
        let attrs = &s.attrs;
        let fields = &s.fields;

        quote! {
            #(#attrs)*
            #[derive(pvm::Encode, pvm::Decode, Default)]
            #vis struct #name #fields

            impl #name {
                const STORAGE_KEY: [u8; 32] = pvm::storage_key(stringify!(#name).as_bytes());

                pub fn load() -> Self {
                    pvm::storage::get_storage_raw(&Self::STORAGE_KEY)
                        .and_then(|bytes| <Self as pvm::Decode>::decode(&mut &bytes[..]).ok())
                        .unwrap_or_default()
                }

                pub fn save(&self) {
                    use pvm::Encode;
                    pvm::storage::set_storage_raw(&Self::STORAGE_KEY, &self.encode());
                }
            }
        }
    }).unwrap_or_default();

    // Generate init function and wrapper
    let (init_code, init_wrapper_name) = init_fn.as_ref().map(|f| {
        let name = &f.sig.ident;
        let vis = &f.vis;
        let attrs = &f.attrs;
        let block = &f.block;
        let inputs = &f.sig.inputs;
        let output = &f.sig.output;
        let wrapper_name = format_ident!("__pvm_init_{}", name);

        let code = quote! {
            #(#attrs)*
            #vis fn #name(#inputs) #output #block

            #[doc(hidden)]
            pub fn #wrapper_name() {
                let state = #name();
                state.save();
            }
        };

        (code, wrapper_name)
    }).unwrap_or_else(|| {
        (quote! {}, format_ident!("__pvm_init_default"))
    });

    // Generate call functions, wrappers, and selectors
    let mut call_codes = Vec::new();
    let mut selector_consts = Vec::new();
    let mut match_arms = Vec::new();

    for func in &call_fns {
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

        let sel_bytes = selector.iter().copied();
        selector_consts.push(quote! {
            const #selector_const: [u8; 4] = [#(#sel_bytes),*];
        });

        // Generate wrapper based on function signature
        let wrapper = generate_call_wrapper(&wrapper_fn, name, inputs, output);

        call_codes.push(quote! {
            #(#attrs)*
            #vis fn #name(#inputs) #output #block

            #wrapper
        });

        match_arms.push(quote! {
            #selector_const => #wrapper_fn()
        });
    }

    // Generate the contract ref struct for cross-contract calls
    let ref_struct = generate_contract_ref(&storage_struct, &call_fns);

    // Generate the entry points
    let deploy_entry = if init_fn.is_some() {
        quote! {
            #[polkavm_derive::polkavm_export]
            extern "C" fn deploy() {
                pvm::alloc_impl::reset();
                #init_wrapper_name();
                pvm::return_value(&[]);
            }
        }
    } else {
        quote! {
            #[polkavm_derive::polkavm_export]
            extern "C" fn deploy() {
                pvm::alloc_impl::reset();
                pvm::return_value(&[]);
            }
        }
    };

    let call_entry = if match_arms.is_empty() {
        quote! {
            #[polkavm_derive::polkavm_export]
            extern "C" fn call() {
                pvm::alloc_impl::reset();
                pvm::revert(b"no methods");
            }
        }
    } else {
        quote! {
            #[polkavm_derive::polkavm_export]
            extern "C" fn call() {
                pvm::alloc_impl::reset();

                let data = pvm::input();
                if data.len() < 4 {
                    pvm::revert(b"no selector");
                }

                let selector: [u8; 4] = [data[0], data[1], data[2], data[3]];

                match selector {
                    #(#match_arms,)*
                    _ => pvm::revert(b"unknown selector"),
                }
            }
        }
    };

    // Generate the complete module with all boilerplate
    quote! {
        // ============================================================
        // PVM Contract Boilerplate (generated by #[pvm::contract])
        // ============================================================

        // On RISC-V target (actual contract): alloc is provided by pvm-core
        #[cfg(target_arch = "riscv64")]
        extern crate alloc;

        // On host target (rust-analyzer, tests): use std
        #[cfg(not(target_arch = "riscv64"))]
        extern crate std;

        #[cfg(not(target_arch = "riscv64"))]
        extern crate alloc;

        // Note: #[global_allocator] and #[panic_handler] are provided by pvm-core
        // They are NOT generated here to avoid conflicts when contracts depend on each other

        // ============================================================
        // Contract Module
        // ============================================================

        #(#mod_attrs)*
        #[allow(dead_code)]  // Suppress warnings on host target (functions used via entry points on riscv64)
        #mod_vis mod #mod_name {
            use super::*;

            #storage_code

            #init_code

            #(#call_codes)*

            #(#other_items)*
        }

        // ============================================================
        // Entry Points (generated by #[pvm::contract])
        // Only compiled for RISC-V target when NOT used as a dependency
        // ============================================================

        #[cfg(all(target_arch = "riscv64", not(feature = "pvm-as-dependency")))]
        use #mod_name::*;

        #[cfg(all(target_arch = "riscv64", not(feature = "pvm-as-dependency")))]
        #(#selector_consts)*

        #[cfg(all(target_arch = "riscv64", not(feature = "pvm-as-dependency")))]
        #deploy_entry

        #[cfg(all(target_arch = "riscv64", not(feature = "pvm-as-dependency")))]
        #call_entry

        // ============================================================
        // Contract Reference (for cross-contract calls)
        // ============================================================

        #ref_struct
    }
}

/// Generate a contract reference struct for cross-contract calls
fn generate_contract_ref(
    storage_struct: &Option<ItemStruct>,
    call_fns: &[ItemFn],
) -> TokenStream {
    let storage_name = match storage_struct {
        Some(s) => &s.ident,
        None => return quote! {},
    };

    let ref_struct_name = format_ident!("{}Ref", storage_name);

    // Generate methods for each #[call] function
    let mut ref_methods = Vec::new();
    let mut ref_selector_consts = Vec::new();

    for func in call_fns {
        let method_name = &func.sig.ident;
        let method_name_str = method_name.to_string();
        let output = &func.sig.output;

        // Compute selector
        let selector = compute_selector(&method_name_str);
        let selector_const = format_ident!("SELECTOR_{}", method_name_str.to_uppercase());
        let sel_bytes = selector.iter().copied();

        ref_selector_consts.push(quote! {
            const #selector_const: [u8; 4] = [#(#sel_bytes),*];
        });

        // Parse arguments, skipping the first one if it's a state reference
        let (arg_names, arg_types) = parse_ref_method_args(&func.sig.inputs);

        // Generate method implementation
        let method_impl = generate_ref_method(
            method_name,
            &selector_const,
            &arg_names,
            &arg_types,
            output,
        );

        ref_methods.push(method_impl);
    }

    quote! {
        /// Contract reference for making cross-contract calls to this contract.
        ///
        /// Use this struct to call methods on a deployed instance of this contract
        /// from another contract.
        #[derive(Clone, Copy)]
        pub struct #ref_struct_name {
            address: pvm::Address,
        }

        impl #ref_struct_name {
            /// Create a new contract reference from an address
            pub fn new(address: pvm::Address) -> Self {
                Self { address }
            }

            /// Get the contract address
            pub fn address(&self) -> pvm::Address {
                self.address
            }

            #(#ref_selector_consts)*

            #(#ref_methods)*
        }
    }
}

/// Parse method arguments, skipping state reference if present
fn parse_ref_method_args(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
) -> (Vec<syn::Ident>, Vec<syn::Type>) {
    let mut names = Vec::new();
    let mut types = Vec::new();
    let mut first = true;

    for arg in inputs {
        if let FnArg::Typed(pat_type) = arg {
            // Skip if this is the first arg and it's a reference (state param)
            if first {
                first = false;
                if let syn::Type::Reference(_) = &*pat_type.ty {
                    continue; // Skip state parameter
                }
            }

            // Extract the argument name
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                names.push(pat_ident.ident.clone());
                types.push((*pat_type.ty).clone());
            }
        }
    }

    (names, types)
}

/// Generate a single method for the contract ref
fn generate_ref_method(
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

fn has_attr(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident(name)
    })
}

fn remove_pvm_attr(mut s: ItemStruct) -> ItemStruct {
    s.attrs.retain(|attr| !attr.path().is_ident("storage"));
    s
}

fn remove_pvm_attr_fn(mut f: ItemFn) -> ItemFn {
    f.attrs.retain(|attr| {
        !attr.path().is_ident("init") && !attr.path().is_ident("call")
    });
    f
}

fn generate_call_wrapper(
    wrapper_fn: &syn::Ident,
    original_fn: &syn::Ident,
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    output: &syn::ReturnType,
) -> TokenStream {
    // Check if first arg is state reference and extract the type
    let state_info = if let Some(syn::FnArg::Typed(pat_type)) = inputs.first() {
        if let syn::Type::Reference(ref_type) = &*pat_type.ty {
            let is_mutable = ref_type.mutability.is_some();
            let state_type = &*ref_type.elem;
            Some((state_type.clone(), is_mutable))
        } else {
            None
        }
    } else {
        None
    };

    if let Some((state_type, is_mutable)) = state_info {
        let call_and_return = match output {
            syn::ReturnType::Default => {
                if is_mutable {
                    quote! {
                        #original_fn(&mut state);
                        state.save();
                        pvm::return_value(&[]);
                    }
                } else {
                    quote! {
                        #original_fn(&state);
                        pvm::return_value(&[]);
                    }
                }
            }
            syn::ReturnType::Type(_, _) => {
                if is_mutable {
                    quote! {
                        let result = #original_fn(&mut state);
                        state.save();
                        let encoded = pvm::Encode::encode(&result);
                        pvm::return_value(&encoded);
                    }
                } else {
                    quote! {
                        let result = #original_fn(&state);
                        let encoded = pvm::Encode::encode(&result);
                        pvm::return_value(&encoded);
                    }
                }
            }
        };

        quote! {
            #[doc(hidden)]
            pub fn #wrapper_fn() {
                let mut state = <#state_type>::load();
                #call_and_return
            }
        }
    } else {
        // No state argument
        let call_and_return = match output {
            syn::ReturnType::Default => quote! {
                #original_fn();
                pvm::return_value(&[]);
            },
            syn::ReturnType::Type(_, _) => quote! {
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
}
