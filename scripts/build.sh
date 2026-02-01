#!/usr/bin/env bash
set -euo pipefail

# Build all contracts in the workspace
# Outputs:
#   target/<name>.<profile>.polkavm   - PolkaVM bytecode
#   target/<name>.<profile>.abi.json  - Solidity-compatible ABI
#   .papi/descriptors/                - TypeScript types (via papi)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

PROFILE="${1:-release}"

echo "Building contracts..."
if [[ "$PROFILE" == "release" ]]; then
    cargo build --release -p name-registry
else
    cargo build -p name-registry
fi

echo ""
echo "Copying ABI to papi contracts folder..."
mkdir -p .papi/contracts
cp "target/name-registry.$PROFILE.abi.json" .papi/contracts/nameRegistry.json

echo ""
echo "Generating TypeScript types with papi..."
npx papi generate

echo ""
echo "Build complete:"
ls -lh "target/name-registry.$PROFILE.polkavm" \
       "target/name-registry.$PROFILE.abi.json" \
       ".papi/descriptors/dist/contracts/nameRegistry.d.ts" 2>/dev/null || true
