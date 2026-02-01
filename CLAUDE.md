# Project: PolkaVM Smart Contracts

## Overview

This project builds PolkaVM smart contracts using `cargo-pvm-contract`. The contracts use Solidity-compatible ABI encoding (not ink! metadata format).

## Project Structure

```
crates/
  utils/src/                  # Shared utilities
    lib.rs                    # Exports allocator module
    allocator.rs              # 256KB global allocator (picoalloc)
  contracts/
    main/src/                 # Name registry contract
      main.rs
.papi/
  contracts/                  # ABI files for papi
  descriptors/dist/           # Generated TypeScript types
    contracts/nameRegistry.d.ts
references/                   # Reference repositories (READ-ONLY)
  cargo-pvm-contract/         # Contract framework
  ink/                        # ink! smart contracts (for reference)
  polkadot-api/               # TypeScript API for Polkadot
```

## Build Commands

```bash
# Full build: contract + ABI + TypeScript types
./scripts/build.sh              # Release (default)
./scripts/build.sh debug        # Debug

# Just regenerate TypeScript types
./scripts/codegen.sh

# Manual cargo build (no TypeScript codegen)
cargo build --release -p name-registry

# Add/update chain metadata for papi
npx papi add <name> -w ws://127.0.0.1:<port>
```

## Build Outputs

| File | Description |
|------|-------------|
| `target/name-registry.release.polkavm` | PolkaVM bytecode (deploy this) |
| `target/name-registry.release.abi.json` | Solidity-compatible ABI |
| `.papi/descriptors/dist/contracts/` | TypeScript types via papi |

## Local Development (Zombienet)

```bash
# Start zombienet
BIN=$(pwd)/bin zombie-cli spawn -p native ./bin/local-dev.toml

# Endpoints:
# - Relay chain:  ws://127.0.0.1:10000
# - Asset Hub:    ws://127.0.0.1:10020
# - People chain: ws://127.0.0.1:10010
# - Bulletin:     ws://127.0.0.1:10030
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
  - `packages/ink-contracts/` - ink! contract support
  - `packages/codegen/src/sol-types.ts` - Solidity ABI → TypeScript codegen
  - `packages/cli/src/commands/sol.ts` - CLI for adding Solidity contracts

### ink
- Location: `references/ink/`
- Purpose: ink! smart contract framework (alternative to cargo-pvm-contract)
- Key files:
  - `crates/metadata/` - ink! metadata format definition
  - `crates/allocator/` - Bump allocator implementation

## TypeScript Codegen (papi)

### How It Works
1. `cargo build` generates Solidity-compatible ABI JSON
2. ABI is copied to `.papi/contracts/`
3. `npx papi generate` creates TypeScript types in `.papi/descriptors/`

### Generated Types
```typescript
// .papi/descriptors/dist/contracts/nameRegistry.d.ts
type MessagesDescriptor = {
  "register": { message: { "name": bigint }, response: {} },
  "lookup": { message: { "addr": Address }, response: bigint },
  "myName": { message: {}, response: bigint }
}
```

### Adding New Contracts
```bash
# Add contract ABI to papi
npx papi sol add target/my-contract.release.abi.json myContract

# Regenerate types
npx papi generate
```

### Using Generated Types
```typescript
import { contracts } from "@polkadot-api/descriptors"
const { nameRegistry } = contracts
```

## Notes

### Allocator
- Uses picoalloc with 256KB heap (configurable in `crates/utils/src/allocator.rs`)
- The allocator is pulled in via `extern crate utils;` in contracts
- Similar pattern to ink!'s allocator crate

### Storage Keys
- Uses 32-byte keys with namespace prefix + address
- Current implementation XORs address bytes into key (see `compute_key` function)
- Alternative: hash-based keys (more flexible, standard in Solidity/ink!)

### Contract Attributes
Use fully-qualified attribute paths for ABI generation to work:
```rust
#[pvm_contract::contract]  // NOT #[contract]
#[pvm_contract::method]    // NOT #[method]
#[pvm_contract::constructor]
```
