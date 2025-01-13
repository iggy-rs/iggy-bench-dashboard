#!/bin/bash

# Exit on any error
set -e

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# Go up one level to frontend directory
FRONTEND_DIR="$(dirname "$SCRIPT_DIR")"

if [ "$TRUNK_PROFILE" = "debug" ]; then
    echo "Debug build detected, using index.dev.html"
    cp "$FRONTEND_DIR/index.dev.html" "$FRONTEND_DIR/index.html"
elif [ "$TRUNK_PROFILE" = "release" ]; then
    echo "Release build detected, using index.prod.html"
    cp "$FRONTEND_DIR/index.prod.html" "$FRONTEND_DIR/index.html"
else
    echo "Error: TRUNK_PROFILE environment variable must be set to 'debug' or 'release'"
    exit 1
fi
