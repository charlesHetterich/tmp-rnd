//! PVM Core - Runtime library for PVM smart contracts
//!
//! This crate provides the low-level primitives for interacting with the
//! PolkaVM runtime: storage, host functions, types, and memory allocation.

#![no_std]

extern crate alloc;

pub mod alloc_impl;
pub mod host;
pub mod storage;
pub mod address;

// Re-export SCALE codec
pub use parity_scale_codec::{Decode, Encode};

// Core types - addresses are generic to support both EVM and Substrate
pub use address::{Address, EvmAddress, SubstrateAddress, AccountId};

/// 32-byte hash type
pub type Hash = [u8; 32];

/// Balance type (128-bit for large values)
pub type Balance = u128;

/// Timestamp in milliseconds
pub type Timestamp = u64;

/// Selector type (first 4 bytes of hash)
pub type Selector = [u8; 4];

// Convenience re-exports from host module
pub use host::{caller, now, value_transferred, address as contract_address, block_number};
pub use host::{return_value, revert, input};
pub use storage::{get_storage, set_storage, remove_storage, contains_storage};

/// Event trait for contract events
pub trait Event: Encode {
    /// Returns the topics for this event (indexed fields)
    fn topics(&self) -> alloc::vec::Vec<[u8; 32]>;
}

/// Emit an event
pub fn emit<E: Event>(event: E) {
    let topics = event.topics();
    let data = event.encode();
    host::emit_event(&topics, &data);
}

/// Compute a selector from a function name (first 4 bytes of blake2_256)
pub const fn selector_from_name(name: &[u8]) -> Selector {
    // Simple compile-time hash - just use first 4 bytes of name for now
    // TODO: Use proper blake2 at compile time
    let mut sel = [0u8; 4];
    let len = if name.len() < 4 { name.len() } else { 4 };
    let mut i = 0;
    while i < len {
        sel[i] = name[i];
        i += 1;
    }
    sel
}

/// Compute a storage key from a name (32 bytes)
/// Uses a simple const-compatible hash for compile-time evaluation
pub const fn storage_key(name: &[u8]) -> Hash {
    // Simple deterministic key derivation
    // XOR the name bytes across the 32-byte key with rotation
    let mut key = [0u8; 32];
    let mut i = 0;
    while i < name.len() {
        key[i % 32] ^= name[i];
        key[(i + 7) % 32] ^= name[i].wrapping_add(i as u8);
        i += 1;
    }
    // Add a distinguishing prefix
    key[0] = key[0].wrapping_add(0x70); // 'p' for pvm
    key[1] = key[1].wrapping_add(0x76); // 'v'
    key[2] = key[2].wrapping_add(0x6d); // 'm'
    key
}
