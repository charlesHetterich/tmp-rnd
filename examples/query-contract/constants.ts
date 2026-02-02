// Generous defaults - work for most contracts, unused gas is refunded
export const GAS_LIMIT = { ref_time: 500_000_000_000n, proof_size: 2_000_000n };

export const STORAGE_DEPOSIT_LIMIT = 10_000_000_000_000n;

// Default signer for dev networks
export const DEFAULT_SIGNER = "Alice";

// Default WebSocket URL for local development
export const DEFAULT_NODE_URL = "ws://127.0.0.1:10020";
