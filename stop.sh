#!/usr/bin/env bash

# Ports used by zombienet
PORTS=(9944 9945 9946 9947)

echo "Stopping zombienet and freeing ports..."

# Kill any processes using our ports
for port in "${PORTS[@]}"; do
    pids=$(lsof -ti :$port 2>/dev/null || true)
    if [[ -n "$pids" ]]; then
        echo "Killing processes on port $port: $pids"
        echo "$pids" | xargs kill -9 2>/dev/null || true
    fi
done

# Also kill any zombienet-related processes
pkill -f "polkadot.*--chain.*rococo" 2>/dev/null || true
pkill -f "polkadot-parachain.*--chain" 2>/dev/null || true
pkill -f "zombienet" 2>/dev/null || true

echo "Done. All ports freed."
