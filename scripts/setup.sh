#!/bin/bash
# Setup script for pvm-contracts
# Downloads required toolchain files

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BIN_DIR="${BIN:-$REPO_ROOT/bin}"

mkdir -p "$BIN_DIR"

# PolkaVM target specification
# Source: https://github.com/paritytech/polkavm/tree/master/crates/polkavm-linker/targets
TARGET_SPEC="$BIN_DIR/riscv64emac-unknown-none-polkavm.json"
if [ ! -f "$TARGET_SPEC" ]; then
    echo "Downloading PolkaVM target spec..."
    curl -sSfL -o "$TARGET_SPEC" \
        "https://raw.githubusercontent.com/paritytech/polkavm/master/crates/polkavm-linker/targets/1_91/riscv64emac-unknown-none-polkavm.json"
    echo "Downloaded: $TARGET_SPEC"
else
    echo "Target spec already exists: $TARGET_SPEC"
fi

# Check for polkatool
if ! command -v polkatool &> /dev/null; then
    echo ""
    echo "WARNING: polkatool not found. Install with:"
    echo "  cargo install polkavm-linker"
fi

echo ""
echo "Setup complete. Build with:"
echo "  ./scripts/build.sh flipper"
