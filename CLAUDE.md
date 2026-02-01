# Project: PolkaVM Smart Contracts

## Overview

This project builds PolkaVM smart contracts using `cargo-pvm-contract`. The contracts use Solidity-compatible ABI encoding (not ink! metadata format).

## Project Structure

```
crates/
  utils/                    # Shared utilities
    Cargo.toml
    lib.rs                  # Exports allocator module
    allocator.rs            # 256KB global allocator (picoalloc)
  contracts/
    main/                   # Name registry contract
      Cargo.toml
      build.rs
      main.rs
references/                 # Reference repositories (READ-ONLY)
  cargo-pvm-contract/       # Contract framework
  ink/                      # ink! smart contracts (for reference)
  polkadot-api/             # TypeScript API for Polkadot
```

## Build Commands

```bash
# Build contract (generates .polkavm file)
cargo build --release -p name-registry

# Output location
target/name-registry.release.polkavm
```

## Reference Repositories

**IMPORTANT**: Always check `./references/` for implementation details before searching the web or making assumptions.

### cargo-pvm-contract
- Location: `references/cargo-pvm-contract/`
- Purpose: PolkaVM contract framework with Solidity-compatible ABI
- Key files:
  - `crates/cargo-pvm-contract-builder/src/abi.rs` - ABI generation
  - `crates/pvm-contract-macros/src/signature/types.rs` - Solidity type mapping
  - `crates/pvm-contract-macros/src/codegen/` - Code generation for dispatch, encode, decode

### polkadot-api
- Location: `references/polkadot-api/`
- Purpose: TypeScript API for interacting with Polkadot/Substrate chains
- Key packages:
  - `packages/ink-contracts/` - ink! contract support (reads ink! metadata)
  - `packages/codegen/src/sol-types.ts` - Solidity ABI → TypeScript codegen
  - `packages/cli/src/commands/sol.ts` - CLI for adding Solidity contracts

### ink
- Location: `references/ink/`
- Purpose: ink! smart contract framework (alternative to cargo-pvm-contract)
- Key files:
  - `crates/metadata/` - ink! metadata format definition
  - `crates/allocator/` - Bump allocator implementation

## TypeScript Codegen

### Current Status
cargo-pvm-contract generates **Solidity-compatible ABI JSON** (not ink! metadata).

polkadot-api has:
1. **ink! support** (`@polkadot-api/ink-contracts`) - for ink! metadata format
2. **Solidity support** (`packages/codegen/src/sol-types.ts`) - for Solidity ABI format

### Approach for TypeScript Generation
Since cargo-pvm-contract outputs Solidity-style ABI, use polkadot-api's Solidity codegen:
```bash
# polkadot-api CLI (if available)
papi sol add contract.abi.json contractName
```

Or implement custom codegen based on `references/polkadot-api/packages/codegen/src/sol-types.ts`.

## Notes

### ABI Generation Issue
The ABI generator in cargo-pvm-contract looks for source files in `{manifest_dir}/src/`. Since our contract uses `main.rs` directly in the crate root (not `src/main.rs`), ABI may not be generated automatically.

### Allocator
- Uses picoalloc with 256KB heap (configurable in `crates/utils/allocator.rs`)
- The allocator is pulled in via `extern crate utils;` in contracts
- Similar pattern to ink!'s allocator crate

### Storage Keys
- Uses 32-byte keys with namespace prefix + address
- Current implementation XORs address bytes into key (see `compute_key` function)
- Alternative: hash-based keys (more flexible, standard in Solidity/ink!)
