//! Address types for PVM contracts
//!
//! Provides address types for cross-contract calls and account identification.
//! Follows ink! v6 conventions where `Address` is a 20-byte EVM-compatible type
//! (equivalent to Solidity's `address` / Ethereum's H160).

use parity_scale_codec::{Decode, Encode};

/// 20-byte EVM-compatible address (equivalent to H160/Solidity `address`).
/// This is the standard address type for pallet-revive contracts.
#[derive(Clone, Copy, Default, PartialEq, Eq, Encode, Decode)]
pub struct Address(pub [u8; 20]);

impl Address {
    /// Address length in bytes
    pub const LEN: usize = 20;

    /// Create a zero address
    pub fn zero() -> Self {
        Self([0u8; 20])
    }

    /// Check if this is the zero address
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

    /// Get the raw bytes of this address
    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    /// Get a mutable reference to the raw bytes
    pub fn as_bytes_mut(&mut self) -> &mut [u8; 20] {
        &mut self.0
    }
}

impl From<[u8; 20]> for Address {
    fn from(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl core::fmt::Debug for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x")?;
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

// Type aliases for backwards compatibility
pub type EvmAddress = Address;
pub type AccountId = Address;

/// Substrate-compatible 32-byte address (AccountId32)
/// Use this when interacting with Substrate-native contracts or pallets.
#[derive(Clone, Copy, Default, PartialEq, Eq, Encode, Decode)]
pub struct SubstrateAddress(pub [u8; 32]);

impl SubstrateAddress {
    /// Address length in bytes
    pub const LEN: usize = 32;

    /// Create a zero address
    pub fn zero() -> Self {
        Self([0u8; 32])
    }

    /// Check if this is the zero address
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

    /// Get the raw bytes of this address
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Get a mutable reference to the raw bytes
    pub fn as_bytes_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}

impl From<[u8; 32]> for SubstrateAddress {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl AsRef<[u8]> for SubstrateAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl core::fmt::Debug for SubstrateAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x")?;
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// Convert Address (20-byte) to SubstrateAddress (32-byte) by left-padding with zeros
impl From<Address> for SubstrateAddress {
    fn from(addr: Address) -> Self {
        let mut bytes = [0u8; 32];
        bytes[12..32].copy_from_slice(&addr.0);
        Self(bytes)
    }
}

/// Try to convert SubstrateAddress to Address (only works if first 12 bytes are zeros)
impl TryFrom<SubstrateAddress> for Address {
    type Error = ();

    fn try_from(substrate: SubstrateAddress) -> Result<Self, ()> {
        if substrate.0[0..12].iter().all(|&b| b == 0) {
            let mut bytes = [0u8; 20];
            bytes.copy_from_slice(&substrate.0[12..32]);
            Ok(Self(bytes))
        } else {
            Err(())
        }
    }
}
