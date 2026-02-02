#![no_main]
#![no_std]

extern crate utils;

use pvm_contract as pvm;
use pvm::{Address, U256, caller};
use utils::storage::Mapping;

/// Simple name registry contract
/// Allows users to register and look up names associated with addresses
#[pvm::contract]
mod name_registry {
    use super::*;
    use pvm_contract::api;

    // Storage definition inside the contract module
    #[pvm::storage]
    struct Storage {
        registration_count: u64,
        names: Mapping<[u8; 20], [u8; 32]>,
    }

    /// Initialize the contract
    #[pvm::constructor]
    pub fn new() -> Result<(), Error> {
        caller();
        Storage::registration_count().set(&0u64);
        Ok(())
    }

    /// Register a name for the caller's address
    #[pvm::method]
    pub fn register(name: U256) {
        let caller = get_caller();
        let caller_bytes: [u8; 20] = caller.as_slice().try_into().unwrap();

        // Store the name for this address
        Storage::names().insert(&caller_bytes, &name.to_be_bytes::<32>());

        // Increment registration count
        let count = Storage::registration_count().get().unwrap_or(0);
        Storage::registration_count().set(&(count + 1));
    }

    /// Look up the name registered to an address
    #[pvm::method]
    pub fn lookup(addr: Address) -> U256 {
        let addr_bytes: [u8; 20] = addr.as_slice().try_into().unwrap();
        Storage::names()
            .get(&addr_bytes)
            .map(|bytes| U256::from_be_bytes(bytes))
            .unwrap_or(U256::ZERO)
    }

    /// Get the name registered to the caller
    #[pvm::method]
    pub fn my_name() -> U256 {
        let caller = get_caller();
        lookup(caller)
    }

    /// Get the total number of registrations
    #[pvm::method]
    pub fn total_registrations() -> u64 {
        Storage::registration_count().get().unwrap_or(0)
    }

    fn get_caller() -> Address {
        let mut addr = [0u8; 20];
        api::caller(&mut addr);
        Address::from(addr)
    }
}
