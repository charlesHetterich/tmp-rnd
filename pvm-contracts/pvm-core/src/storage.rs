//! Storage helpers for PVM contracts
//!
//! Provides typed access to contract storage using SCALE encoding.

use crate::Hash;
use parity_scale_codec::{Decode, Encode};

#[cfg(target_arch = "riscv64")]
use uapi::{HostFn, HostFnImpl as api, StorageFlags};

#[cfg(target_arch = "riscv64")]
use crate::alloc_impl;

/// Maximum value size for storage operations
pub const MAX_VALUE_SIZE: usize = 16 * 1024;

/// Get raw bytes from storage
pub fn get_storage_raw(key: &[u8]) -> Option<&'static [u8]> {
    #[cfg(target_arch = "riscv64")]
    {
        // Allocate buffer for the value
        let buffer = alloc_impl::alloc(MAX_VALUE_SIZE)?;

        // Read from storage
        let mut output: &mut [u8] = buffer;
        let result = api::get_storage(StorageFlags::empty(), key, &mut output);

        match result {
            Ok(_) => {
                let len = output.len();
                Some(&buffer[..len])
            }
            Err(_) => None,
        }
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = key;
        None
    }
}

/// Set raw bytes in storage
pub fn set_storage_raw(key: &[u8], value: &[u8]) {
    #[cfg(target_arch = "riscv64")]
    {
        api::set_storage(StorageFlags::empty(), key, value);
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = (key, value);
    }
}

/// Remove a key from storage
pub fn remove_storage(key: &[u8]) {
    #[cfg(target_arch = "riscv64")]
    {
        api::set_storage(StorageFlags::empty(), key, &[]);
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = key;
    }
}

/// Check if a key exists in storage
pub fn contains_storage(key: &[u8]) -> bool {
    #[cfg(target_arch = "riscv64")]
    {
        let mut buffer = [0u8; 1];
        let mut output: &mut [u8] = &mut buffer;
        api::get_storage(StorageFlags::empty(), key, &mut output).is_ok()
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = key;
        false
    }
}

/// Get a SCALE-encoded value from storage
pub fn get_storage<T: Decode>(key: &[u8]) -> Option<T> {
    let bytes = get_storage_raw(key)?;
    T::decode(&mut &bytes[..]).ok()
}

/// Set a SCALE-encoded value in storage
pub fn set_storage<T: Encode>(key: &[u8], value: &T) {
    let encoded = value.encode();
    set_storage_raw(key, &encoded);
}

/// Compute a storage key from multiple parts using simple XOR mixing
/// For production, use blake2 hashing
pub fn storage_key(parts: &[&[u8]]) -> Hash {
    let mut key = [0u8; 32];

    for (i, part) in parts.iter().enumerate() {
        for (j, byte) in part.iter().enumerate() {
            let idx = (i * 7 + j) % 32;
            key[idx] ^= byte;
        }
    }

    key
}

/// Compute a storage key at compile time (simplified version)
pub const fn storage_key_const(name: &[u8]) -> Hash {
    let mut key = [0u8; 32];

    // Simple hash: spread bytes across the key
    let mut i = 0;
    while i < name.len() && i < 32 {
        key[i] = name[i];
        i += 1;
    }

    // Mix remaining bytes
    while i < name.len() {
        key[i % 32] ^= name[i];
        i += 1;
    }

    key
}
