//! Flipper User - Example contract that calls another contract (Flipper)
//!
//! This demonstrates cross-contract calls in PVM by interacting with a
//! deployed Flipper contract.
//!
//! Usage:
//! 1. Deploy a Flipper contract
//! 2. Deploy this FlipperUser contract
//! 3. Call `set_flipper_address` with the Flipper contract address
//! 4. Call `call_flip` to flip the value in the Flipper contract
//! 5. Call `call_get` to read the value from the Flipper contract

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pvm::contract]
mod flipper_user {
    use pvm::{call_contract, call_contract_with_output, Address};

    /// Selector for "flip" function (first 4 bytes of name)
    /// This matches how the #[call] macro generates selectors
    const FLIP_SELECTOR: [u8; 4] = [b'f', b'l', b'i', b'p'];

    /// Selector for "get" function (first 4 bytes of name)
    const GET_SELECTOR: [u8; 4] = [b'g', b'e', b't', 0];

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
        // Build the call data (just the selector for flip)
        let call_data = FLIP_SELECTOR.to_vec();

        // Call the Flipper contract with no value transfer
        let _ = call_contract(&state.flipper_address, 0, &call_data);
    }

    /// Call the get() method on the Flipper contract and return the result
    /// Returns the current boolean value stored in the Flipper
    #[call]
    pub fn call_get(state: &FlipperUser) -> bool {
        // Build the call data (just the selector for get)
        let call_data = GET_SELECTOR.to_vec();

        // Buffer to receive the output
        let mut output = [0u8; 32];

        // Call the Flipper contract and get the return value
        match call_contract_with_output(&state.flipper_address, 0, &call_data, &mut output) {
            Ok(_) => {
                // Decode the bool from SCALE-encoded output
                // A SCALE-encoded bool is 1 byte: 0x00 for false, 0x01 for true
                output[0] != 0
            }
            Err(_) => false,
        }
    }

    /// Get the stored flipper contract address
    #[call]
    pub fn get_flipper_address(state: &FlipperUser) -> Address {
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
