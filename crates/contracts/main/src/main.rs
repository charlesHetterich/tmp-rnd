#![no_main]
#![no_std]

extern crate utils;

use pvm_contract as pvm;
use pvm::{Address, caller};
use pvm::storage::Mapping;
use alloc::string::String;

/// Allows users to register and look up names associated with addresses
#[pvm::contract]
mod name_registry {
    use super::*;

    #[pvm::storage]
    struct Storage {
        registration_count: u64,
        names: Mapping<Address, String>,
    }

    /// Initialize the contract
    #[pvm::constructor]
    pub fn new() -> Result<(), Error> {
        Storage::registration_count().set(&0);
        Ok(())
    }

    /// Register a name for the caller's address
    #[pvm::method]
    pub fn register(name: String) {
        Storage::names().insert(&caller(), &name);

        let count = Storage::registration_count().get().unwrap_or(0);
        Storage::registration_count().set(&(count + 1));
    }

    /// Look up the name registered to an address
    #[pvm::method]
    pub fn lookup(addr: Address) -> String {
        Storage::names()
            .get(&addr)
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
