//! PVM Macros - Procedural macros for PVM smart contracts
//!
//! This crate provides the following macros:
//! - `#[pvm::storage]` - Marks a struct as contract storage
//! - `#[pvm::init]` - Marks a function as the contract constructor
//! - `#[pvm::call]` - Marks a function as callable
//! - `#[pvm::event]` - Marks a struct as an event

use proc_macro::TokenStream;

mod storage;
mod call;
mod init;
mod event;

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

/// Helper to compute a simple selector from a name
pub(crate) fn compute_selector(name: &str) -> [u8; 4] {
    use blake2::{Blake2s256, Digest};
    let mut hasher = Blake2s256::new();
    hasher.update(name.as_bytes());
    let result = hasher.finalize();
    [result[0], result[1], result[2], result[3]]
}
