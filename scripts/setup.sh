#!/bin/bash

set -euo pipefail

echo "Setting up the environment"

# Start InfluxDB service first
docker compose up -d influxdb

# Wait until InfluxDB is ready
until docker exec influxdb influx ping; do
  echo "Waiting for InfluxDB to be ready..."
  sleep 2
done

# If .env file is not present, create tokens for Grafana and InfluxDB
if [ ! -f .env ]; then
  echo "Generating InfluxDB token..."

  # Create InfluxDB token with all-access
  INFLUXDB_TOKEN=$(docker exec influxdb influx auth create --org iggy --all-access | awk 'NR==2 {print $2}')

  echo "INFLUXDB_TOKEN=${INFLUXDB_TOKEN}" > .env
  echo "Created InfluxDB admin token: ${INFLUXDB_TOKEN}"
fi

# shellcheck disable=SC1091
source .env

# Start Grafana service
docker compose up -d grafana

# Wait until Grafana is ready
until curl -s -o /dev/null http://localhost:3001/api/health; do
  echo "Waiting for Grafana to be ready..."
  sleep 2
done

echo "Grafana is up and running with InfluxDB data source and dashboards."
