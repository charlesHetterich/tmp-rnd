# Zombie Network Setup

A Rust-based tool for spawning multi-chain Polkadot networks using [zombienet-sdk](https://github.com/paritytech/zombienet-sdk).

## What This Spawns

This tool creates two networks:

### Main Network
- **Westend Relay Chain** - 5 validators
- **Asset Hub** (parachain 1000) - with Revive support
- **People Chain** (parachain 1004) - with PoP & statement store
- **Template Chain** (parachain 2001) - generic parachain template

### Bulletin Network
- **Bulletin Chain** - standalone chain for bulletin/statement storage

## Quick Start

```bash
# Clone this repo
git clone <your-repo-url>
cd zombie-setup

# Run setup (downloads pre-built binaries + builds bulletin-chain)
./setup.sh

# Start the networks
./run.sh
```

## Requirements

- **Rust** (stable) - [Install](https://rustup.rs/) - needed for bulletin-chain and zombie-setup
- **Git**
- **curl**

## Setup Options

```bash
# Default: download pre-built polkadot binaries, build bulletin-chain
./setup.sh

# Build everything from source (slower but works on all platforms)
./setup.sh --build-polkadot

# Only download polkadot binaries (skip bulletin-chain)
./setup.sh --download-only

# Skip bulletin-chain if you don't need the bulletin network
./setup.sh --skip-bulletin
```

### Pre-built Binaries

The setup script downloads pre-built binaries from [polkadot-sdk releases](https://github.com/paritytech/polkadot-sdk/releases):

| Platform | Status |
|----------|--------|
| Linux x86_64 | ✅ Supported |
| macOS ARM64 (Apple Silicon) | ✅ Supported |
| macOS x86_64 | Build from source |
| Linux ARM64 | Build from source |

Note: `polkadot-bulletin-chain` has no pre-built binaries and must be built from source (~10-20 min).

## Manual Setup

If you prefer to set things up manually:

```bash
# 1. Download pre-built Polkadot binaries (macOS ARM64 example)
mkdir -p bin
curl -L https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2512/polkadot-aarch64-apple-darwin -o bin/polkadot
curl -L https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2512/polkadot-parachain-aarch64-apple-darwin -o bin/polkadot-parachain
curl -L https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2512/polkadot-execute-worker-aarch64-apple-darwin -o bin/polkadot-execute-worker
curl -L https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2512/polkadot-prepare-worker-aarch64-apple-darwin -o bin/polkadot-prepare-worker
chmod +x bin/*

# 2. Build polkadot-bulletin-chain (no pre-built available)
git clone --depth 1 https://github.com/paritytech/polkadot-bulletin-chain.git
cd polkadot-bulletin-chain
cargo build --release -p polkadot-bulletin-chain
cp target/release/polkadot-bulletin-chain ../bin/
cd ..

# 3. Add binaries to PATH and run
export PATH="$(pwd)/bin:$PATH"
cargo run --release
```

## Runtime Files

The `runtime/` directory contains the WASM runtime files for each chain:

| File | Chain |
|------|-------|
| `polkadot_runtime-v2000004.compact.compressed.wasm` | Relay chain |
| `asset-hub-polkadot_runtime-v2000004.compact.compressed.wasm` | Asset Hub |
| `people_rococo_runtime.compact.compressed.wasm` | People chain |
| `parachain_template_runtime.compact.compressed.wasm` | Template chain |

## Configuration

Edit `src/config.rs` to customize:
- Binary paths/commands
- Runtime file paths
- Add/remove chains

Edit `src/main.rs` to customize:
- Parachain IDs
- Number of validators/collators
- Node arguments and logging levels
- Genesis overrides

## Adding More Chains

To add a new parachain:

1. Add the runtime WASM file to `runtime/`
2. Add the runtime path to `ChainConfig` in `src/config.rs`
3. Add the parachain configuration in `generate_config_main_network()` in `src/main.rs`:

```rust
.with_parachain(|p| {
    p.with_id(YOUR_PARA_ID)
        .with_chain("your-chain-local")
        .with_chain_spec_runtime(
            chain_config.your_runtime_path.as_str(),
            Some("local"),
        )
        .with_default_command(chain_config.parachain_command.as_str())
        .with_collator(|c| {
            c.with_name("collator-your-chain")
                .with_args(vec!["-lbasic-authorship=trace".into()])
        })
})
```

## Accessing the Networks

Once running, the console will print URLs for each node:
- **Polkadot.js Apps**: `https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:<port>#/explorer`
- **PAPI Explorer**: `https://dev.papi.how/explorer#networkId=custom&endpoint=ws://127.0.0.1:<port>`

## Troubleshooting

### Binary not found
Make sure the Polkadot binaries are in your PATH or run `./run.sh` which sets up the PATH automatically.

### Runtime file not found
Check that all required `.wasm` files exist in the `runtime/` directory.

### Build fails
- Ensure you have the latest Rust: `rustup update`
- Ensure wasm target is installed: `rustup target add wasm32-unknown-unknown`

### macOS security warnings
On first run, macOS may block the binaries. Go to **Settings > Privacy & Security** and click "Allow Anyway" for each blocked binary.

## License

MIT
