//! PVM - Lightweight macro framework for PVM smart contracts
//!
//! This crate provides a simple, ergonomic way to write smart contracts
//! for PolkaVM using Rust.
//!
//! # Example
//!
//! ```ignore
//! use pvm::*;
//!
//! #[pvm::storage]
//! pub struct Counter {
//!     count: u32,
//! }
//!
//! #[pvm::init]
//! pub fn new() -> Counter {
//!     Counter { count: 0 }
//! }
//!
//! #[pvm::call]
//! pub fn increment(state: &mut Counter) {
//!     state.count += 1;
//! }
//!
//! #[pvm::call]
//! pub fn get(state: &Counter) -> u32 {
//!     state.count
//! }
//! ```

#![no_std]

extern crate alloc;

// Re-export macros
pub use pvm_macros::{storage, init, call, event};

// Re-export core types and functions
pub use pvm_core::{
    // Address types (generic address support)
    Address, EvmAddress, SubstrateAddress, AccountId,
    // Other types
    Hash, Balance, Timestamp, Selector,
    // SCALE codec
    Encode, Decode,
    // Host functions
    caller, now, value_transferred, contract_address, block_number,
    return_value, revert, input,
    // Storage
    get_storage, set_storage, remove_storage, contains_storage,
    // Events
    Event, emit,
    // Allocator
    alloc_impl,
};

// Re-export alloc for use in generated code
pub use alloc::vec;
pub use alloc::vec::Vec;
