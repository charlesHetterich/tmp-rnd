#![no_main]
#![no_std]

use pvm_contract::{contract, constructor, method, Address, U256, caller};
use xxhash_rust::const_xxh32::xxh32;

#[global_allocator]
static mut ALLOC: picoalloc::Mutex<picoalloc::Allocator<picoalloc::ArrayPointer<1024>>> = {
    static mut ARRAY: picoalloc::Array<1024> = picoalloc::Array([0u8; 1024]);

    picoalloc::Mutex::new(picoalloc::Allocator::new(unsafe {
        picoalloc::ArrayPointer::new(&raw mut ARRAY)
    }))
};

/// Simple name registry contract
/// Allows users to register and look up names associated with addresses
#[contract]
mod name_registry {
    use super::*;
    use pvm_contract::{api, StorageFlags};

    // Storage key prefix for names mapping
    const NAMES_PREFIX: [u8; 32] = [
        0x6e, 0x61, 0x6d, 0x65, 0x73, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    /// Initialize the contract
    #[constructor]
    pub fn new() {
        // api::caller()
        caller();

        // Nothing to initialize
    }

    /// Register a name for the caller's address
    /// The name is encoded as a U256 (max 32 bytes)
    #[method]
    pub fn register(name: U256) {
        let caller = get_caller();
        let key = compute_key(&caller);
        let value = name.to_be_bytes::<32>();
        api::hash_keccak_256(input, output);
        api::set_storage(StorageFlags::empty(), &key, &value);
    }

    /// Look up the name registered to an address
    /// Returns U256(0) if no name is registered
    #[method]
    pub fn lookup(addr: Address) -> U256 {
        let key = compute_key(&addr);
        let mut value = [0u8; 32];
        let mut out = value.as_mut_slice();
        let _ = api::get_storage(StorageFlags::empty(), &key, &mut out);
        U256::from_be_bytes(value)
    }

    /// Get the name registered to the caller
    #[method]
    pub fn my_name() -> U256 {
        let caller = get_caller();
        lookup(caller)
    }

    /// Helper: get the caller address
    fn get_caller() -> Address {
        let mut addr = [0u8; 20];
        api::caller(&mut addr);
        Address::from(addr)
    }

    /// Helper: compute storage key from address
    fn compute_key(addr: &Address) -> [u8; 32] {
        let mut key = NAMES_PREFIX;
        let addr_bytes = addr.as_slice();
        // XOR address bytes into the key (offset by 12 to fill the 32-byte key)
        for i in 0..20 {
            key[12 + i] ^= addr_bytes[i];
        }
        key
    }
}
