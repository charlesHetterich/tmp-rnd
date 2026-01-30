//! Address types for PVM contracts
//!
//! Supports both EVM (20-byte) and Substrate (32-byte) addresses through
//! a generic Address trait and concrete implementations.

use parity_scale_codec::{Decode, Encode};

/// Generic address trait that can represent different address formats
pub trait Address: Encode + Decode + Clone + Default + PartialEq + Eq {
    /// The raw bytes type for this address
    type Bytes: AsRef<[u8]> + AsMut<[u8]> + Default + Clone;

    /// Create an address from raw bytes
    fn from_bytes(bytes: Self::Bytes) -> Self;

    /// Get the raw bytes of this address
    fn as_bytes(&self) -> &Self::Bytes;

    /// Get a mutable reference to the raw bytes
    fn as_bytes_mut(&mut self) -> &mut Self::Bytes;

    /// The length of this address in bytes
    const LEN: usize;

    /// Create a zero address
    fn zero() -> Self {
        Self::from_bytes(Self::Bytes::default())
    }

    /// Check if this is the zero address
    fn is_zero(&self) -> bool {
        self.as_bytes().as_ref().iter().all(|&b| b == 0)
    }
}

/// EVM-compatible 20-byte address
#[derive(Clone, Copy, Default, PartialEq, Eq, Encode, Decode)]
pub struct EvmAddress(pub [u8; 20]);

impl Address for EvmAddress {
    type Bytes = [u8; 20];

    fn from_bytes(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }

    fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    fn as_bytes_mut(&mut self) -> &mut [u8; 20] {
        &mut self.0
    }

    const LEN: usize = 20;
}

impl From<[u8; 20]> for EvmAddress {
    fn from(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }
}

impl AsRef<[u8]> for EvmAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl core::fmt::Debug for EvmAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x")?;
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// Substrate-compatible 32-byte address (AccountId32)
#[derive(Clone, Copy, Default, PartialEq, Eq, Encode, Decode)]
pub struct SubstrateAddress(pub [u8; 32]);

impl Address for SubstrateAddress {
    type Bytes = [u8; 32];

    fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    fn as_bytes_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }

    const LEN: usize = 32;
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

/// Convert between address types
impl From<EvmAddress> for SubstrateAddress {
    fn from(evm: EvmAddress) -> Self {
        let mut bytes = [0u8; 32];
        // Left-pad with zeros
        bytes[12..32].copy_from_slice(&evm.0);
        Self(bytes)
    }
}

impl TryFrom<SubstrateAddress> for EvmAddress {
    type Error = ();

    fn try_from(substrate: SubstrateAddress) -> Result<Self, ()> {
        // Only valid if first 12 bytes are zeros
        if substrate.0[0..12].iter().all(|&b| b == 0) {
            let mut bytes = [0u8; 20];
            bytes.copy_from_slice(&substrate.0[12..32]);
            Ok(Self(bytes))
        } else {
            Err(())
        }
    }
}

// For backward compatibility, provide a type alias
// Users can configure which address type to use via features or their own type alias

/// Default address type - EVM-compatible 20 bytes
/// This matches pallet-revive's address format
pub type AccountId = EvmAddress;
