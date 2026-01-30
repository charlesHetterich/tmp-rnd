//! PVM Macros - Procedural macros for PVM smart contracts
//!
//! This crate provides the following macros:
//! - `#[pvm::contract]` - Transforms a module into a complete PVM contract
//! - `#[storage]` - Marks a struct as contract storage (inside contract module)
//! - `#[init]` - Marks a function as the contract constructor
//! - `#[call]` - Marks a function as callable
//! - `#[event]` - Marks a struct as an event

use proc_macro::TokenStream;

mod contract;
mod storage;
mod call;
mod init;
mod event;
mod interface;

/// Transforms a module into a complete PVM smart contract.
///
/// This macro generates all necessary boilerplate including:
/// - Global allocator
/// - Panic handler
/// - `call` and `deploy` entry points
/// - Selector dispatch logic
///
/// # Example
/// ```ignore
/// #![no_std]
/// #![no_main]
///
/// #[pvm::contract]
/// mod my_contract {
///     #[storage]
///     pub struct Counter { count: u32 }
///
///     #[init]
///     pub fn new() -> Counter { Counter { count: 0 } }
///
///     #[call]
///     pub fn increment(state: &mut Counter) { state.count += 1; }
///
///     #[call]
///     pub fn get(state: &Counter) -> u32 { state.count }
/// }
/// ```
#[proc_macro_attribute]
pub fn contract(_attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::expand(item.into()).into()
}

/// Marks a struct as the contract's storage.
///
/// This macro:
/// - Adds `Encode`, `Decode`, and `Default` derives
/// - Generates a `STORAGE_KEY` constant
/// - Generates `load()` and `save()` methods
///
/// # Example
/// ```ignore
/// #[pvm::storage]
/// pub struct Counter {
///     count: u32,
/// }
/// ```
#[proc_macro_attribute]
pub fn storage(_attr: TokenStream, item: TokenStream) -> TokenStream {
    storage::expand(item.into()).into()
}

/// Marks a function as the contract's constructor.
///
/// This function is called during contract deployment.
/// It must return the storage struct.
///
/// # Example
/// ```ignore
/// #[pvm::init]
/// pub fn new(initial: u32) -> Counter {
///     Counter { count: initial }
/// }
/// ```
#[proc_macro_attribute]
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    init::expand(item.into()).into()
}

/// Marks a function as callable (part of the contract's public API).
///
/// This macro:
/// - Generates a selector based on the function name
/// - Generates a wrapper that loads storage, calls the function, and saves storage
/// - Registers the function for dispatch
///
/// # Example
/// ```ignore
/// #[pvm::call]
/// pub fn increment(state: &mut Counter) {
///     state.count += 1;
/// }
/// ```
#[proc_macro_attribute]
pub fn call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    call::expand(item.into()).into()
}

/// Marks a struct as an event.
///
/// Events can be emitted using `pvm::emit(event)`.
///
/// # Example
/// ```ignore
/// #[pvm::event]
/// pub struct Incremented {
///     new_value: u32,
/// }
/// ```
#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    event::expand(item.into()).into()
}

/// Transforms a trait into a contract reference struct for cross-contract calls.
///
/// This macro generates a `{TraitName}Ref` struct that wraps an address and
/// provides type-safe methods for calling the contract.
///
/// # Example
/// ```ignore
/// #[pvm::interface]
/// pub trait Flipper {
///     fn flip();
///     fn get() -> bool;
/// }
///
/// // Usage in another contract:
/// let flipper = FlipperRef::new(flipper_address);
/// let value = flipper.get()?;
/// flipper.flip()?;
/// ```
#[proc_macro_attribute]
pub fn interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    interface::expand(item.into()).into()
}

/// Helper to compute a simple selector from a name
pub(crate) fn compute_selector(name: &str) -> [u8; 4] {
    use blake2::{Blake2s256, Digest};
    let mut hasher = Blake2s256::new();
    hasher.update(name.as_bytes());
    let result = hasher.finalize();
    [result[0], result[1], result[2], result[3]]
}
