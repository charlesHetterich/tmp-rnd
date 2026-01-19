#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN="$DIR/bin"
SDK_DIR="$DIR/.polkadot-sdk"
BULLETIN_DIR="$DIR/.polkadot-bulletin-chain"

mkdir -p "$BIN"

install_system_deps() {
    echo "Checking system dependencies..."

    # Detect OS
    if [[ "$(uname -s)" == "Linux" ]]; then
        # Check if we need to install packages
        local MISSING_PKGS=()

        dpkg -s libclang-dev &>/dev/null || MISSING_PKGS+=(libclang-dev)
        dpkg -s llvm &>/dev/null || MISSING_PKGS+=(llvm)
        dpkg -s libssl-dev &>/dev/null || MISSING_PKGS+=(libssl-dev)
        dpkg -s pkg-config &>/dev/null || MISSING_PKGS+=(pkg-config)
        dpkg -s cmake &>/dev/null || MISSING_PKGS+=(cmake)
        dpkg -s protobuf-compiler &>/dev/null || MISSING_PKGS+=(protobuf-compiler)
        dpkg -s git &>/dev/null || MISSING_PKGS+=(git)
        dpkg -s curl &>/dev/null || MISSING_PKGS+=(curl)

        if [[ ${#MISSING_PKGS[@]} -gt 0 ]]; then
            echo "Installing missing packages: ${MISSING_PKGS[*]}"
            sudo apt-get update
            sudo apt-get install -y "${MISSING_PKGS[@]}"
        else
            echo "All system packages are installed."
        fi
    elif [[ "$(uname -s)" == "Darwin" ]]; then
        # macOS - use Homebrew
        if ! command -v brew &>/dev/null; then
            echo "Error: Homebrew is required on macOS. Install from https://brew.sh"
            exit 1
        fi
        brew install llvm openssl cmake protobuf git curl 2>/dev/null || true
    fi
}

install_rust() {
    if ! command -v rustup &>/dev/null; then
        echo "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo "Rust is already installed."
    fi
}

install_zombienet() {
    if command -v zombienet &>/dev/null; then
        echo "Zombienet is already installed: $(zombienet version)"
        return
    fi

    # Fetch latest version from GitHub API
    local ZOMBIENET_VERSION
    ZOMBIENET_VERSION=$(curl -s https://api.github.com/repos/paritytech/zombienet/releases/latest | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

    if [[ -z "$ZOMBIENET_VERSION" ]]; then
        echo "Error: Could not fetch latest zombienet version from GitHub"
        exit 1
    fi

    echo "Installing zombienet ${ZOMBIENET_VERSION} (latest)..."

    local ARCH
    local OS
    local ZOMBIENET_BIN

    case "$(uname -s)" in
        Linux)  OS="linux" ;;
        Darwin) OS="macos" ;;
        *)      echo "Unsupported OS"; exit 1 ;;
    esac

    case "$(uname -m)" in
        x86_64)  ARCH="x64" ;;
        aarch64|arm64) ARCH="arm64" ;;
        *)       echo "Unsupported architecture"; exit 1 ;;
    esac

    ZOMBIENET_BIN="zombienet-${OS}-${ARCH}"

    curl -L -o /tmp/zombienet "https://github.com/paritytech/zombienet/releases/download/${ZOMBIENET_VERSION}/${ZOMBIENET_BIN}"
    chmod +x /tmp/zombienet

    # Try to install to /usr/local/bin, fall back to ~/.local/bin
    if sudo mv /tmp/zombienet /usr/local/bin/zombienet 2>/dev/null; then
        echo "Zombienet installed to /usr/local/bin/zombienet"
    else
        mkdir -p "$HOME/.local/bin"
        mv /tmp/zombienet "$HOME/.local/bin/zombienet"
        echo "Zombienet installed to ~/.local/bin/zombienet"
        echo "Make sure ~/.local/bin is in your PATH"
        export PATH="$HOME/.local/bin:$PATH"
    fi

    zombienet version
}

setup_build_env() {
    # macOS: ensure LLVM/clang is available for wasm builds
    if [[ "$(uname -s)" == "Darwin" ]] && command -v brew &>/dev/null; then
        LLVM=$(brew --prefix llvm 2>/dev/null || true)
        [[ -d "$LLVM/lib" ]] && export LIBCLANG_PATH="$LLVM/lib"
    fi

    # Linux: set LIBCLANG_PATH if not set
    if [[ "$(uname -s)" == "Linux" ]] && [[ -z "$LIBCLANG_PATH" ]]; then
        # Common locations for libclang on Linux
        for path in /usr/lib/llvm-18/lib /usr/lib/llvm-17/lib /usr/lib/llvm-16/lib /usr/lib/llvm-15/lib /usr/lib/llvm-14/lib /usr/lib/x86_64-linux-gnu /usr/lib64; do
            if [[ -f "$path/libclang.so" ]] || ls "$path"/libclang-*.so &>/dev/null 2>&1; then
                export LIBCLANG_PATH="$path"
                echo "Set LIBCLANG_PATH=$LIBCLANG_PATH"
                break
            fi
        done
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

# Install all dependencies first
install_system_deps
install_rust
install_zombienet
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
