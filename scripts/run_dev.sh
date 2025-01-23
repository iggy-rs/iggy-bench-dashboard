#!/bin/bash

# Exit on any error
set -e

# Function to cleanup background processes
cleanup() {
    echo "Shutting down services..."
    kill "$(jobs -p)" 2>/dev/null
    exit 0
}

# Set up trap for SIGINT (Ctrl+C)
trap cleanup SIGINT

# Parse arguments
TRUNK_ARGS=""
if [ $# -gt 0 ]; then
    if [ "$1" = "open" ]; then
        TRUNK_ARGS="--open"
    else
        echo "Error: Invalid argument. Only 'open' is supported."
        exit 1
    fi
fi

# Start server in background
echo "Starting server..."
(cd "$(dirname "$0")/.." && cargo run --bin iggy-benchmarks-dashboard-server) &

# Wait a bit for server to start
sleep 1

# Start frontend in background
echo "Starting frontend..."
(cd "$(dirname "$0")/.." && trunk serve --config frontend/Trunk.toml $TRUNK_ARGS) &

# Wait for all background processes
wait
