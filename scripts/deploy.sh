#!/usr/bin/env bash
set -euo pipefail

# Deploy contract to local Asset Hub via eth-rpc
# Uses Alith dev account (pre-funded in local dev chains)

ALITH_KEY="0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"
RPC="${RPC:-http://127.0.0.1:8545}"
CONTRACT="${1:-target/name-registry.release.polkavm}"

cast send --private-key "$ALITH_KEY" --rpc-url "$RPC" --create "$(cat "$CONTRACT" | xxd -p | tr -d '\n')"
