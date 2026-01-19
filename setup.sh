#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN="$DIR/bin"
SDK_DIR="$DIR/.polkadot-sdk"
BULLETIN_DIR="$DIR/.polkadot-bulletin-chain"

mkdir -p "$BIN"

setup_build_env() {
    # macOS: ensure LLVM/clang is available for wasm builds
    if [[ "$(uname -s)" == Darwin* ]] && command -v brew &>/dev/null; then
        LLVM=$(brew --prefix llvm 2>/dev/null || true)
        [[ -d "$LLVM/lib" ]] && export LIBCLANG_PATH="$LLVM/lib"
    fi
    rustup target add wasm32-unknown-unknown 2>/dev/null || true
}

clone_sdk_repo() {
    if [[ -d "$SDK_DIR" ]]; then
        echo "Updating polkadot-sdk..."
        cd "$SDK_DIR" && git pull --ff-only && cd "$DIR"
    else
        echo "Cloning polkadot-sdk (this may take a while)..."
        git clone --depth 1 https://github.com/paritytech/polkadot-sdk.git "$SDK_DIR"
    fi
}

build_sdk_binaries() {
    echo "Building polkadot-sdk binaries (this will take a while on first run)..."

    # Check if binaries already exist and are up to date
    if [[ -f "$BIN/polkadot" && -f "$BIN/polkadot-parachain" ]]; then
        echo "Binaries already exist. To rebuild, delete bin/ directory."
        return
    fi

    cd "$SDK_DIR"

    # Build relay chain node (workers are built as part of this)
    echo "Building polkadot (relay chain + workers)..."
    cargo build --release -p polkadot

    # Build parachain node
    echo "Building polkadot-parachain..."
    cargo build --release -p polkadot-parachain-bin

    # Copy binaries
    cp target/release/polkadot "$BIN/"
    cp target/release/polkadot-parachain "$BIN/"
    cp target/release/polkadot-execute-worker "$BIN/"
    cp target/release/polkadot-prepare-worker "$BIN/"

    cd "$DIR"
    echo "SDK binaries built successfully!"
}

clone_bulletin_repo() {
    if [[ -d "$BULLETIN_DIR" ]]; then
        echo "Updating polkadot-bulletin-chain..."
        cd "$BULLETIN_DIR" && git pull --ff-only && cd "$DIR"
    else
        echo "Cloning polkadot-bulletin-chain..."
        git clone --depth 1 https://github.com/paritytech/polkadot-bulletin-chain.git "$BULLETIN_DIR"
    fi
}

build_bulletin_parachain_runtime() {
    local WASM="$BULLETIN_DIR/target/release/wbuild/bulletin-westend-runtime/bulletin_westend_runtime.compact.compressed.wasm"
    [[ -f "$WASM" ]] && return

    echo "Building bulletin-westend-runtime..."
    cargo build --release --manifest-path "$BULLETIN_DIR/Cargo.toml" -p bulletin-westend-runtime
}

generate_bulletin_spec() {
    [[ -f "$BIN/bulletin-parachain-spec.json" ]] && return

    echo "Generating bulletin parachain chain spec..."
    command -v chain-spec-builder &>/dev/null || { echo "Installing chain-spec-builder..."; cargo install staging-chain-spec-builder; }

    local WASM="$BULLETIN_DIR/target/release/wbuild/bulletin-westend-runtime/bulletin_westend_runtime.compact.compressed.wasm"

    cd "$BIN"
    chain-spec-builder create \
        -p 1006 \
        -c rococo \
        -i bulletin-local \
        -n "Bulletin Parachain" \
        -t local \
        -r "$WASM" \
        named-preset local_testnet

    mv chain_spec.json bulletin-parachain-spec.json
}

# Main
echo "==================================="
echo "Polkadot Local Dev Environment Setup"
echo "==================================="
echo ""

setup_build_env

# Build SDK from source (relay + parachain nodes)
clone_sdk_repo
build_sdk_binaries

# Build bulletin chain parachain runtime
clone_bulletin_repo
build_bulletin_parachain_runtime
generate_bulletin_spec

echo ""
echo "==================================="
echo "Setup complete!"
echo "==================================="
echo ""
echo "Binaries in: $BIN"
$BIN/polkadot --version
$BIN/polkadot-parachain --version
echo ""
echo "To start zombienet:"
echo "  ./run.sh"
echo ""
