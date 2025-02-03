#!/bin/sh
set -e

# Debug info
echo "Checking performance_results directory:"
ls -la /data/performance_results
echo "Current user:"
id

exec /app/iggy-bench-dashboard-server \
    --host "${HOST}" \
    --port "${PORT}" \
    --results-dir "${RESULTS_DIR}"
