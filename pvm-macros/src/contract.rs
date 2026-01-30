//! Contract macro implementation
//!
//! The `#[pvm::contract]` attribute transforms a module into a complete PVM contract,
//! generating all necessary boilerplate.

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{parse2, ItemMod, Item, ItemFn, ItemStruct, Attribute};

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

        // On RISC-V target (actual contract): use custom allocator, no_std
        #[cfg(target_arch = "riscv64")]
        extern crate alloc;

        #[cfg(target_arch = "riscv64")]
        #[global_allocator]
        static __PVM_ALLOCATOR: pvm::alloc_impl::PvmAllocator = pvm::alloc_impl::PvmAllocator;

        #[cfg(target_arch = "riscv64")]
        #[panic_handler]
        fn __pvm_panic(_info: &core::panic::PanicInfo) -> ! {
            unsafe {
                core::arch::asm!("unimp");
                core::hint::unreachable_unchecked()
            }
        }

        // On host target (rust-analyzer, tests): use std
        #[cfg(not(target_arch = "riscv64"))]
        extern crate std;

        #[cfg(not(target_arch = "riscv64"))]
        extern crate alloc;

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
        // Only compiled for RISC-V target
        // ============================================================

        #[cfg(target_arch = "riscv64")]
        use #mod_name::*;

        #[cfg(target_arch = "riscv64")]
        #(#selector_consts)*

        #[cfg(target_arch = "riscv64")]
        #deploy_entry

        #[cfg(target_arch = "riscv64")]
        #call_entry
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
