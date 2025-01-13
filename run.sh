#!/bin/bash

# Exit on any error
set -e

echo "Building frontend..."
cd frontend
trunk build

echo "Starting server..."
cd ..
cargo run -p iggy-dashboard-server
