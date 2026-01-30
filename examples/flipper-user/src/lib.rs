//! Flipper User - Example contract that calls another contract (Flipper)
//!
//! This demonstrates cross-contract calls in PVM by importing `FlipperRef`
//! from the Flipper contract crate.
//!
//! Usage:
//! 1. Deploy a Flipper contract
//! 2. Deploy this FlipperUser contract
//! 3. Call `set_flipper_address` with the Flipper contract address
//! 4. Call `call_flip` to flip the value in the Flipper contract
//! 5. Call `call_get` to read the value from the Flipper contract

#![cfg_attr(not(feature = "std"), no_std, no_main)]

// Import FlipperRef from the Flipper contract crate
use flipper::FlipperRef;

#[pvm::contract]
mod flipper_user {
    use super::FlipperRef;
    use pvm::Address;

    #[storage]
    pub struct FlipperUser {
        /// The address of the Flipper contract to interact with
        flipper_address: Address,
    }

    /// Create a new FlipperUser contract
    /// After deployment, call `set_flipper_address` to configure the target Flipper
    #[init]
    pub fn new() -> FlipperUser {
        FlipperUser {
            flipper_address: Address::default(),
        }
    }

    /// Call the flip() method on the Flipper contract
    /// This will toggle the boolean value stored in the Flipper
    #[call]
    pub fn call_flip(state: &FlipperUser) {
        let flipper = FlipperRef::new(state.flipper_address);
        // Ignore errors for simplicity in this example
        let _ = flipper.flip();
    }

    /// Call the get() method on the Flipper contract and return the result
    /// Returns the current boolean value stored in the Flipper
    #[call]
    pub fn call_get(state: &FlipperUser) -> bool {
        let flipper = FlipperRef::new(state.flipper_address);
        // Return false on error
        flipper.get().unwrap_or(false)
    }

    /// Get the stored flipper contract address
    #[call]
    pub fn get_flipper_address(state: &FlipperUser) -> pvm::Address {
        state.flipper_address
    }

    /// Update the flipper contract address
    /// Must be called after deployment to configure which Flipper to interact with
    #[call]
    pub fn set_flipper_address(state: &mut FlipperUser) {
        // TODO: Once macro supports arguments, this will take the address as a param
        // For now, parse the address from raw input after the selector
        let input = pvm::input();
        if input.len() >= 24 {
            // Skip the 4-byte selector, read 20 bytes for the address
            let mut addr_bytes = [0u8; 20];
            addr_bytes.copy_from_slice(&input[4..24]);
            state.flipper_address = Address::from(addr_bytes);
        }
    }
}
