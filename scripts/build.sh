#!/bin/bash
# Build PVM contracts
#
# Usage: ./scripts/build.sh [contract_name]
# Example: ./scripts/build.sh flipper

set -e

# Find repo root (where Cargo.toml is)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Use $BIN if set, otherwise use local bin/
BIN_DIR="${BIN:-$REPO_ROOT/bin}"
TARGET_SPEC="$BIN_DIR/riscv64emac-unknown-none-polkavm.json"

if [ ! -f "$TARGET_SPEC" ]; then
    echo "Error: Target spec not found at $TARGET_SPEC"
    echo "Run ./scripts/setup.sh first"
    exit 1
fi

CONTRACT="${1:-flipper}"

echo "Building $CONTRACT for PolkaVM..."

RUSTC_BOOTSTRAP=1 cargo +nightly build --release \
  --target "$TARGET_SPEC" \
  -p "$CONTRACT" \
  -Zbuild-std=core,alloc

echo "Linking to .polkavm..."
# Find the build output (could be .so, .elf, or no extension depending on toolchain)
BUILD_DIR="$REPO_ROOT/target/riscv64emac-unknown-none-polkavm/release"
if [ -f "$BUILD_DIR/lib${CONTRACT}.so" ]; then
  INPUT="$BUILD_DIR/lib${CONTRACT}.so"
elif [ -f "$BUILD_DIR/${CONTRACT}.elf" ]; then
  INPUT="$BUILD_DIR/${CONTRACT}.elf"
elif [ -f "$BUILD_DIR/$CONTRACT" ]; then
  INPUT="$BUILD_DIR/$CONTRACT"
else
  echo "Error: Could not find build output"
  ls -la "$BUILD_DIR/" | grep -i "$CONTRACT"
  exit 1
fi

OUTPUT="$REPO_ROOT/$CONTRACT.polkavm"
polkatool link "$INPUT" -o "$OUTPUT"

echo ""
echo "Success! Output: $OUTPUT"
ls -la "$OUTPUT"
