#!/bin/bash

# Exit on any error
set -e

# Build frontend
trunk build --release --config frontend/Trunk.toml

# Build server
cargo build --release --bin iggy-dashboard-server
