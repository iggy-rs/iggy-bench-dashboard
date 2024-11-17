use crate::{args::IggyDashboardArgs, db::client::IggyDashboardInfluxDbClient};
use anyhow::{Context, Result};

/// Application to process performance results of iggy.
/// This executable has multiple subcommands:
///   - Poll GitHub for new artifacts and download them, GITHUB_TOKEN environment
///     variable must be set
///   - Ingest performance results from a directory or GitHub artifact and add data
///     to local InfluxDB - INFLUXDB_TOKEN environment variable must be set
///   - Run benchmarks locally and ingest the results into InfluxDB
///
/// InfluxDB and grafana should be started using docker-compose up -d in the root of the repository.

pub struct IggyDashboardApp {
    args: IggyDashboardArgs,
    db: IggyDashboardInfluxDbClient,
}

impl IggyDashboardApp {
    pub fn new(args: IggyDashboardArgs) -> Result<Self> {
        let db = IggyDashboardInfluxDbClient::new()?;
        Ok(Self { args, db })
    }

    pub fn run(&self) -> Result<()> {
        // let mut dir = fs::read_dir(path).await?;

        // while let Some(child) = dir.next_entry().await? {
        //     if child.metadata().await?.is_dir() {
        //         println!("Processing: {:?}", child.path().display());
        //         let data =
        //             BenchmarkSummary::new(child.path().to_str().unwrap(), &commit_or_tag).unwrap();
        //         client.query(data.into_query("performance_metrics")).await?;
        //     }
        // }
        Ok(())
    }
}
