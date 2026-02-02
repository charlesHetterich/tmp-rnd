#![no_main]
#![no_std]

extern crate utils;

use pvm_contract as pvm;
use pvm::{Address, caller};
use utils::storage::Mapping;
use alloc::string::String;

/// Simple name registry contract
/// Allows users to register and look up names associated with addresses
#[pvm::contract]
mod name_registry {
    use super::*;

    #[pvm::storage]
    struct Storage {
        registration_count: u64,
        names: Mapping<[u8; 20], String>,
    }

    /// Initialize the contract
    #[pvm::constructor]
    pub fn new() -> Result<(), Error> {
        Storage::registration_count().set(&0u64);
        Ok(())
    }

    /// Register a name for the caller's address
    #[pvm::method]
    pub fn register(name: String) {
        let caller = caller();
        let caller_bytes: [u8; 20] = caller.as_slice().try_into().unwrap();

        Storage::names().insert(&caller_bytes, &name);

        let count = Storage::registration_count().get().unwrap_or(0);
        Storage::registration_count().set(&(count + 1));
    }

    /// Look up the name registered to an address
    #[pvm::method]
    pub fn lookup(addr: Address) -> String {
        let addr_bytes: [u8; 20] = addr.as_slice().try_into().unwrap();
        Storage::names()
            .get(&addr_bytes)
            .unwrap_or_default()
    }

    /// Get the name registered to the caller
    #[pvm::method]
    pub fn my_name() -> String {
        lookup(caller())
    }

    /// Get the total number of registrations
    #[pvm::method]
    pub fn total_registrations() -> u64 {
        Storage::registration_count().get().unwrap_or(0)
    }
}
