use anyhow::{Context, Result};
use influxdb::Client;

const INFLUXDB_URL: &str = "http://localhost:8086";
const INFLUXDB_DB: &str = "iggy-performance-metrics";

pub struct IggyDashboardInfluxDbClient {
    client: Client,
}

impl IggyDashboardInfluxDbClient {
    pub fn new() -> Result<Self> {
        let token = std::env::var("INFLUXDB_TOKEN").context("INFLUXDB_TOKEN not set")?;
        Ok(Self {
            client: Client::new(INFLUXDB_URL, INFLUXDB_DB).with_token(token),
        })
    }
}
