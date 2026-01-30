//! Cross-contract call utilities
//!
//! Provides error types and helpers for making cross-contract calls
//! with automatic encoding and decoding.

use alloc::vec::Vec;
use parity_scale_codec::Decode;

use crate::address::Address;
use crate::host::{call_contract, call_contract_with_output, MAX_OUTPUT_SIZE};
use crate::Balance;

/// Error type for cross-contract calls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractCallError {
    /// The contract call failed (reverted or other error)
    CallFailed,
    /// Failed to decode the return value
    DecodeFailed,
}

/// Build call data from a selector and encoded arguments
///
/// # Arguments
/// * `selector` - The 4-byte function selector
/// * `args` - SCALE-encoded arguments (can be empty slice for no args)
///
/// # Returns
/// A vector containing [selector | args]
pub fn encode_call_data(selector: [u8; 4], args: &[u8]) -> Vec<u8> {
    let mut data = Vec::with_capacity(4 + args.len());
    data.extend_from_slice(&selector);
    data.extend_from_slice(args);
    data
}

/// Call a contract with no return value
///
/// # Arguments
/// * `address` - Target contract address
/// * `selector` - Function selector
/// * `args` - SCALE-encoded arguments
///
/// # Returns
/// `Ok(())` on success, `Err(ContractCallError::CallFailed)` on failure
pub fn call(
    address: &Address,
    selector: [u8; 4],
    args: &[u8],
) -> Result<(), ContractCallError> {
    let data = encode_call_data(selector, args);
    call_contract(address, 0, &data).map_err(|_| ContractCallError::CallFailed)
}

/// Call a contract with a value transfer and no return value
///
/// # Arguments
/// * `address` - Target contract address
/// * `value` - Amount of native token to transfer
/// * `selector` - Function selector
/// * `args` - SCALE-encoded arguments
///
/// # Returns
/// `Ok(())` on success, `Err(ContractCallError::CallFailed)` on failure
pub fn call_with_value(
    address: &Address,
    value: Balance,
    selector: [u8; 4],
    args: &[u8],
) -> Result<(), ContractCallError> {
    let data = encode_call_data(selector, args);
    call_contract(address, value, &data).map_err(|_| ContractCallError::CallFailed)
}

/// Call a contract and decode the return value
///
/// # Arguments
/// * `address` - Target contract address
/// * `selector` - Function selector
/// * `args` - SCALE-encoded arguments
///
/// # Returns
/// `Ok(T)` with decoded return value, or error
pub fn call_and_decode<T: Decode>(
    address: &Address,
    selector: [u8; 4],
    args: &[u8],
) -> Result<T, ContractCallError> {
    let data = encode_call_data(selector, args);
    let mut output = [0u8; MAX_OUTPUT_SIZE];

    call_contract_with_output(address, 0, &data, &mut output)
        .map_err(|_| ContractCallError::CallFailed)?;

    T::decode(&mut &output[..]).map_err(|_| ContractCallError::DecodeFailed)
}

/// Call a contract with value transfer and decode the return value
///
/// # Arguments
/// * `address` - Target contract address
/// * `value` - Amount of native token to transfer
/// * `selector` - Function selector
/// * `args` - SCALE-encoded arguments
///
/// # Returns
/// `Ok(T)` with decoded return value, or error
pub fn call_with_value_and_decode<T: Decode>(
    address: &Address,
    value: Balance,
    selector: [u8; 4],
    args: &[u8],
) -> Result<T, ContractCallError> {
    let data = encode_call_data(selector, args);
    let mut output = [0u8; MAX_OUTPUT_SIZE];

    call_contract_with_output(address, value, &data, &mut output)
        .map_err(|_| ContractCallError::CallFailed)?;

    T::decode(&mut &output[..]).map_err(|_| ContractCallError::DecodeFailed)
}
