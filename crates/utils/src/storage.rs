//! Storage abstraction layer for PolkaVM smart contracts.
//!
//! Provides type-safe storage operations with automatic SCALE encoding/decoding
//! and key hashing.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use utils::storage::{hash_key, get, set};
//!
//! // Create a key by hashing data
//! let key = hash_key(&("balances", user_address));
//!
//! // Store a value
//! set(&key, &1000u128);
//!
//! // Retrieve a value
//! let balance: Option<u128> = get(&key);
//! ```

use core::marker::PhantomData;
use pallet_revive_uapi::{HostFn, HostFnImpl as api, StorageFlags};
use parity_scale_codec::{Decode, Encode};

/// Default buffer size for reading values from storage.
const DEFAULT_READ_BUFFER_SIZE: usize = 512;

/// A 32-byte storage key.
pub type StorageKey = [u8; 32];

// ============================================================================
// Key Hashing
// ============================================================================

/// Hash any SCALE-encodable data into a 32-byte storage key using Keccak256.
///
/// # Example
/// ```rust,ignore
/// let key = hash_key(&("balances", user_address));
/// ```
pub fn hash_key<T: Encode>(data: &T) -> StorageKey {
    let encoded = data.encode();
    let mut key = [0u8; 32];
    api::hash_keccak_256(&encoded, &mut key);
    key
}

/// Combine a namespace with a key to create a namespaced storage key.
///
/// This hashes `(namespace, key)` together using Keccak256.
///
/// # Example
/// ```rust,ignore
/// let key = namespaced_key(b"balances", &user_address);
/// ```
pub fn namespaced_key<K: Encode>(namespace: &[u8], key: &K) -> StorageKey {
    let mut data = namespace.encode();
    key.encode_to(&mut data);

    let mut result = [0u8; 32];
    api::hash_keccak_256(&data, &mut result);
    result
}

/// Use a raw 32-byte array as a storage key (no hashing).
///
/// Use this when you already have a pre-computed key.
#[inline]
pub const fn raw_key(bytes: [u8; 32]) -> StorageKey {
    bytes
}

// ============================================================================
// Value Operations
// ============================================================================

/// Get a value from storage with the given key.
///
/// Returns `None` if:
/// - The key does not exist in storage
/// - The stored value cannot be decoded as type `T`
///
/// # Example
/// ```rust,ignore
/// let balance: Option<u128> = get(&balance_key);
/// ```
pub fn get<T: Decode>(key: &StorageKey) -> Option<T> {
    get_with_buffer::<T, DEFAULT_READ_BUFFER_SIZE>(key)
}

/// Get a value from storage with a custom buffer size.
///
/// Use this when you know the maximum encoded size of your type
/// and want to optimize stack usage or handle larger values.
pub fn get_with_buffer<T: Decode, const N: usize>(key: &StorageKey) -> Option<T> {
    let mut buffer = [0u8; N];
    let mut output = buffer.as_mut_slice();

    match api::get_storage(StorageFlags::empty(), key, &mut output) {
        Ok(_) => {
            // output slice was truncated to actual length by the API
            let bytes_read = N - output.len();
            T::decode(&mut &buffer[..bytes_read]).ok()
        }
        Err(_) => None,
    }
}

/// Set a value in storage at the given key.
///
/// # Example
/// ```rust,ignore
/// set(&balance_key, &1000u128);
/// ```
pub fn set<T: Encode>(key: &StorageKey, value: &T) {
    let encoded = value.encode();
    api::set_storage(StorageFlags::empty(), key, &encoded);
}

/// Remove a value from storage.
///
/// This sets the storage to an empty value, effectively deleting it.
pub fn remove(key: &StorageKey) {
    api::set_storage(StorageFlags::empty(), key, &[]);
}

/// Check if a key exists in storage.
///
/// Returns `true` if a value is stored at the key.
pub fn contains(key: &StorageKey) -> bool {
    let mut buffer = [0u8; 1];
    let mut output = buffer.as_mut_slice();
    api::get_storage(StorageFlags::empty(), key, &mut output).is_ok()
}

// ============================================================================
// Lazy<V> - Single Value Storage
// ============================================================================

/// A single value stored at a fixed storage key.
///
/// "Lazy" because values are not cached - each access goes directly to storage.
///
/// # Example
/// ```rust,ignore
/// let counter: Lazy<u64> = Lazy::new(b"counter");
/// counter.set(&42);
/// let val = counter.get();  // Option<u64>
/// ```
pub struct Lazy<V> {
    key: StorageKey,
    _marker: PhantomData<V>,
}

impl<V> Lazy<V> {
    /// Create a new Lazy storage item with the given namespace.
    /// The namespace is hashed to produce the storage key.
    pub fn new(namespace: &[u8]) -> Self {
        Self {
            key: hash_key(&namespace),
            _marker: PhantomData,
        }
    }

    /// Create a new Lazy storage item with a raw pre-computed key.
    pub fn from_key(key: StorageKey) -> Self {
        Self {
            key,
            _marker: PhantomData,
        }
    }
}

impl<V: Decode> Lazy<V> {
    /// Get the value from storage.
    /// Returns `None` if the key doesn't exist or decoding fails.
    pub fn get(&self) -> Option<V> {
        get(&self.key)
    }
}

impl<V: Encode> Lazy<V> {
    /// Set the value in storage.
    pub fn set(&self, value: &V) {
        set(&self.key, value)
    }
}

impl<V> Lazy<V> {
    /// Remove the value from storage.
    pub fn clear(&self) {
        remove(&self.key)
    }

    /// Check if a value exists at this key.
    pub fn exists(&self) -> bool {
        contains(&self.key)
    }
}

// ============================================================================
// Mapping<K, V> - Key-Value Storage
// ============================================================================

/// A mapping from keys to values, similar to a HashMap but backed by contract storage.
///
/// Each entry is stored at `hash(namespace + key)`.
///
/// # Example
/// ```rust,ignore
/// let balances: Mapping<Address, u128> = Mapping::new(b"balances");
/// balances.insert(&addr, &1000u128);
/// let bal = balances.get(&addr);  // Option<u128>
/// ```
pub struct Mapping<K, V> {
    namespace: StorageKey,
    _marker: PhantomData<(K, V)>,
}

impl<K, V> Mapping<K, V> {
    /// Create a new Mapping with the given namespace.
    /// The namespace is hashed to produce a base key.
    pub fn new(namespace: &[u8]) -> Self {
        Self {
            namespace: hash_key(&namespace),
            _marker: PhantomData,
        }
    }
}

impl<K: Encode, V> Mapping<K, V> {
    /// Compute the storage key for a given map key.
    fn storage_key(&self, key: &K) -> StorageKey {
        // Hash the namespace key together with the encoded user key
        let mut data = self.namespace.encode();
        key.encode_to(&mut data);

        let mut result = [0u8; 32];
        api::hash_keccak_256(&data, &mut result);
        result
    }
}

impl<K: Encode, V: Decode> Mapping<K, V> {
    /// Get the value for a key.
    /// Returns `None` if the key doesn't exist or decoding fails.
    pub fn get(&self, key: &K) -> Option<V> {
        get(&self.storage_key(key))
    }
}

impl<K: Encode, V: Encode> Mapping<K, V> {
    /// Insert a value at the given key.
    pub fn insert(&self, key: &K, value: &V) {
        set(&self.storage_key(key), value)
    }
}

impl<K: Encode, V> Mapping<K, V> {
    /// Remove the value at the given key.
    pub fn remove(&self, key: &K) {
        remove(&self.storage_key(key))
    }

    /// Check if a value exists at the given key.
    pub fn contains(&self, key: &K) -> bool {
        contains(&self.storage_key(key))
    }
}
