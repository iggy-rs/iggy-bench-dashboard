#!/bin/bash

# Exit on any error
set -euo pipefail

# Build frontend
trunk build --release --config frontend/Trunk.toml

# Build server
cargo build --release --bin iggy-bench-dashboard-server
