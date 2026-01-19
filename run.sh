#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export PATH="$DIR/bin:$PATH"

# Ports used by zombienet
PORTS=(9944 9945 9946 9947)

# Kill any processes using our ports
kill_port_users() {
    for port in "${PORTS[@]}"; do
        pids=$(lsof -ti :$port 2>/dev/null || true)
        if [[ -n "$pids" ]]; then
            echo "Killing processes on port $port: $pids"
            echo "$pids" | xargs kill -9 2>/dev/null || true
        fi
    done
    # Brief pause to let ports be released
    sleep 1
}

# Check binaries
for bin in polkadot polkadot-parachain; do
    command -v "$bin" &>/dev/null || { echo "Missing: $bin (run ./setup.sh)"; exit 1; }
done

command -v zombienet &>/dev/null || { echo "Missing: zombienet CLI. Install via: npm install -g @zombienet/cli"; exit 1; }

echo "=============================================="
echo "Starting Zombienet"
echo "=============================================="
echo ""

# Free up ports
echo "Checking for processes using ports ${PORTS[*]}..."
kill_port_users
echo ""

echo "Chains:"
echo "  - Relay Chain (rococo-local)"
echo "  - Asset Hub (parachain 1000)"
echo "  - Bulletin Chain (parachain 1006)"
echo ""
echo "Endpoints:"
echo "  Relay Alice:   ws://127.0.0.1:9944"
echo "  Relay Bob:     ws://127.0.0.1:9945"
echo "  Asset Hub:     ws://127.0.0.1:9946"
echo "  Bulletin:      ws://127.0.0.1:9947"
echo ""
echo "Polkadot.js Apps:"
echo "  Relay:     https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer"
echo "  Asset Hub: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9946#/explorer"
echo "  Bulletin:  https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9947#/explorer"
echo ""
echo "=============================================="
echo ""

cd "$DIR"
zombienet -p native spawn zombienet.toml
