# Project: Polkadot Local Development Environment

## Overview
A local development environment for running Polkadot relay chain + parachains using Zombienet. The goal is to have a working setup with:
- Relay chain (rococo-local)
- Asset Hub (parachain 1000) - with revive pallet for EVM-compatible contracts
- Bulletin Chain (parachain 1006) - for timestamped signed messages (WIP)

## Current Status

### Working
- **Relay chain** - producing blocks via `zombienet-simple.toml`
- **Asset Hub** - producing blocks as parachain 1000
- **PAPI client** - TypeScript client for interacting with chains (`papi-client/`)
  - Uses `withPolkadotSdkCompat` wrapper (required for local Polkadot SDK chains)
  - Connects to both relay chain and Asset Hub
  - Queries balances, runtime versions, subscribes to blocks

### Not Yet Working
- **Bulletin Chain** - SDK version mismatch between:
  - Downloaded binaries: `polkadot 1.21.0` (polkadot-stable2512)
  - Bulletin runtime: built against SDK commit `b2bcb74b13f1a1e082f701e3e05ce1be44d16790`

  Need to either:
  1. Build polkadot-sdk binaries from source matching the bulletin chain's SDK version
  2. Update bulletin chain to use stable2512 and rebuild

## Project Structure

```
tmp-rnd/
├── bin/                          # Downloaded/built binaries
│   ├── polkadot                  # Relay chain node
│   ├── polkadot-parachain        # Parachain collator
│   ├── polkadot-execute-worker
│   ├── polkadot-prepare-worker
│   └── bulletin-parachain-spec.json
├── .polkadot-bulletin-chain/     # Cloned bulletin chain repo (gitignored)
├── papi-client/                  # TypeScript PAPI client
│   ├── src/index.ts
│   └── .papi/                    # Generated descriptors
├── zombienet.toml                # Full config (relay + asset hub + bulletin)
├── zombienet-simple.toml         # Working config (relay + asset hub only)
├── setup.sh                      # Downloads binaries, builds bulletin runtime
├── run.sh                        # Starts zombienet
└── stop.sh                       # Stops zombienet
```

## Key Files

| File | Purpose |
|------|---------|
| `zombienet-simple.toml` | Working zombienet config for relay + asset hub |
| `zombienet.toml` | Full config including bulletin (not working yet) |
| `setup.sh` | Downloads SDK binaries, clones bulletin repo, builds runtime |
| `papi-client/src/index.ts` | Demo script connecting to both chains |

## Running the Environment

```bash
# First time setup (downloads binaries, builds bulletin runtime)
./setup.sh

# Start relay + asset hub only (working)
zombienet -p native spawn zombienet-simple.toml

# Start with bulletin (not working due to version mismatch)
./run.sh
```

## Chain Endpoints (when running)

| Chain | WebSocket | Polkadot.js |
|-------|-----------|-------------|
| Relay (Alice) | ws://127.0.0.1:9944 | [link](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer) |
| Relay (Bob) | ws://127.0.0.1:9945 | [link](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9945#/explorer) |
| Asset Hub | ws://127.0.0.1:9946 | [link](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9946#/explorer) |
| Bulletin | ws://127.0.0.1:9947 | [link](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9947#/explorer) |

## Next Steps

1. ✅ Verify relay + asset hub working with zombienet-simple.toml
2. ✅ Create PAPI client for chain interactions
3. ⬜ Commit checkpoint (relay + asset hub working)
4. ⬜ Fix bulletin chain SDK version mismatch
5. ⬜ Get all 3 chains producing blocks together

## SDK Version Notes

- `polkadot-stable2512` = SDK version used by downloaded binaries
- Bulletin chain repo pins to specific git commit, may need updating
- Asset Hub works because it uses built-in chain spec matching the binary version
