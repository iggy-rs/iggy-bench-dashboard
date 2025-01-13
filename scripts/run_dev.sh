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

# Start server in background
echo "Starting server..."
(cd "$(dirname "$0")/.." && cargo run --bin iggy-dashboard-server) &

# Wait a bit for server to start
sleep 1

# Start frontend in background
echo "Starting frontend..."
(cd "$(dirname "$0")/.." && trunk serve --config frontend/Trunk.toml) &

# Wait for all background processes
wait
