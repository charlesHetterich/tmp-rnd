//! Host function wrappers for PolkaVM
//!
//! These functions wrap the raw host calls provided by pallet-revive-uapi.

use crate::address::EvmAddress;
use crate::Balance;
use crate::Timestamp;

#[cfg(target_arch = "riscv64")]
use uapi::{HostFn, HostFnImpl as api, ReturnFlags};

#[cfg(target_arch = "riscv64")]
use crate::alloc_impl;

/// Maximum input size we support
pub const MAX_INPUT_SIZE: usize = 8 * 1024;

/// Get the caller address
pub fn caller() -> EvmAddress {
    #[cfg(target_arch = "riscv64")]
    {
        let mut addr = [0u8; 20];
        api::caller(&mut addr);
        EvmAddress(addr)
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        EvmAddress::default()
    }
}

/// Get the current contract address
pub fn address() -> EvmAddress {
    #[cfg(target_arch = "riscv64")]
    {
        let mut addr = [0u8; 20];
        api::address(&mut addr);
        EvmAddress(addr)
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        EvmAddress::default()
    }
}

/// Get the current block number
pub fn block_number() -> u64 {
    #[cfg(target_arch = "riscv64")]
    {
        let mut output = [0u8; 32];
        api::block_number(&mut output);
        // Block number is a u64, take last 8 bytes (big-endian)
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&output[24..32]);
        u64::from_be_bytes(bytes)
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        0
    }
}

/// Get the current block timestamp
pub fn now() -> Timestamp {
    #[cfg(target_arch = "riscv64")]
    {
        let mut output = [0u8; 32];
        api::now(&mut output);
        // Timestamp is a u64, take last 8 bytes (big-endian)
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&output[24..32]);
        u64::from_be_bytes(bytes)
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        0
    }
}

/// Get the value transferred with the call
pub fn value_transferred() -> Balance {
    #[cfg(target_arch = "riscv64")]
    {
        let mut output = [0u8; 32];
        api::value_transferred(&mut output);
        // Balance is a u128, take last 16 bytes (big-endian)
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&output[16..32]);
        u128::from_be_bytes(bytes)
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        0
    }
}

/// Get the input data for the current call
pub fn input() -> &'static [u8] {
    #[cfg(target_arch = "riscv64")]
    {
        // Allocate a fixed-size buffer for input
        let buffer = match alloc_impl::alloc(MAX_INPUT_SIZE) {
            Some(b) => b,
            None => return &[],
        };

        // Copy call data into buffer
        api::call_data_copy(buffer, 0);

        // Return the buffer (actual length determined by caller via parsing)
        buffer
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        &[]
    }
}

/// Return data to the caller and exit
pub fn return_value(data: &[u8]) -> ! {
    #[cfg(target_arch = "riscv64")]
    {
        api::return_value(ReturnFlags::empty(), data)
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = data;
        panic!("return_value called outside PolkaVM")
    }
}

/// Revert with error data and exit
pub fn revert(data: &[u8]) -> ! {
    #[cfg(target_arch = "riscv64")]
    {
        api::return_value(ReturnFlags::REVERT, data)
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = data;
        panic!("revert called outside PolkaVM")
    }
}

/// Emit an event with topics and data
pub fn emit_event(topics: &[[u8; 32]], data: &[u8]) {
    #[cfg(target_arch = "riscv64")]
    {
        api::deposit_event(topics, data);
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = (topics, data);
    }
}

/// Call another contract
pub fn call_contract(
    addr: &EvmAddress,
    value: Balance,
    data: &[u8],
) -> Result<(), ()> {
    #[cfg(target_arch = "riscv64")]
    {
        use uapi::CallFlags;

        // Convert value to 32-byte big-endian
        let mut value_bytes = [0u8; 32];
        value_bytes[16..32].copy_from_slice(&value.to_be_bytes());

        let deposit = [0u8; 32]; // No deposit limit
        let mut output: &mut [u8] = &mut [];

        api::call(
            CallFlags::empty(),
            &addr.0,
            0, // ref_time_limit
            0, // proof_size_limit
            &deposit,
            &value_bytes,
            data,
            Some(&mut output),
        ).map_err(|_| ())
    }

    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = (addr, value, data);
        Ok(())
    }
}
