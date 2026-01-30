//! PVM - Lightweight macro framework for PVM smart contracts
//!
//! This crate provides a simple, ergonomic way to write smart contracts
//! for PolkaVM using Rust.
//!
//! # Example
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! #[pvm::contract]
//! mod my_contract {
//!     #[storage]
//!     pub struct Counter { count: u32 }
//!
//!     #[init]
//!     pub fn new() -> Counter { Counter { count: 0 } }
//!
//!     #[call]
//!     pub fn increment(state: &mut Counter) { state.count += 1; }
//!
//!     #[call]
//!     pub fn get(state: &Counter) -> u32 { state.count }
//! }
//! ```

#![no_std]

extern crate alloc;

// Re-export macros
pub use pvm_macros::{contract, storage, init, call, event};

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
    // Cross-contract calls
    call_contract, call_contract_with_output, MAX_OUTPUT_SIZE,
    // Storage module
    storage,
    // Storage key helper
    storage_key,
    // Events
    Event, emit,
    // Allocator
    alloc_impl,
};

// Re-export alloc for use in generated code
pub use alloc::vec;
pub use alloc::vec::Vec;
