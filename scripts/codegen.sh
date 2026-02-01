#!/usr/bin/env bash
set -euo pipefail

# Regenerate TypeScript types using papi
# Requires: ABI file in .papi/contracts/

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

npx papi generate
