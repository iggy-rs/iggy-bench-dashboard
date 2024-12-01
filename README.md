# iggy-dashboard

This repository consists of Rust data ingester and docker compose file for InfluxDB and Grafana.

- Poll GitHub for new artifacts and download them, GITHUB_TOKEN environment variable must be set
- Ingest performance results from a directory or GitHub artifact and add data to local InfluxDB
  `INFLUXDB_TOKEN` environment variable must be set
- Run benchmarks locally and ingest the results into InfluxDB

InfluxDB and grafana should be started using docker-compose up -d in the root of the repository.

## Requirements

```bash
sudo apt install git docker docker-compose
```

## Usage

After cloning the repository, start the docker containers:

```bash
docker-compose up -d
```

Build and run the data ingester:

```bash
cargo build --release
GITHUB_TOKEN=your_github_token \
INFLUXDB_TOKEN=your_influxdb_token \
/target/release/iggy-dashboard
```
