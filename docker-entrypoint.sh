#!/bin/bash
set -e

echo "Checking performance_results directory:"
ls -la /data/performance_results
echo "Current user:"
id

ARGS=()
ARGS+=(--host "${HOST}")
ARGS+=(--port "${PORT}")
ARGS+=(--results-dir "${RESULTS_DIR}")

if [ -n "${GITHUB_TOKEN}" ]; then
    echo "Polling GitHub enabled, GITHUB_TOKEN set"
    ARGS+=(poll-github)
else
    echo "Polling GitHub disabled, GITHUB_TOKEN not set"
fi

exec /app/iggy-bench-dashboard-server "${ARGS[@]}"
