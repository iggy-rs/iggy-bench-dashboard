use super::response::InfluxResponse;
use anyhow::Result;
use influxdb::{Client, Query, ReadQuery};
use rust_dotenv::dotenv::DotEnv;

const INFLUXDB_URL: &str = "http://localhost:8086";
const INFLUXDB_DB: &str = "iggy";

pub struct IggyDashboardInfluxDbClient {
    client: Client,
}

impl IggyDashboardInfluxDbClient {
    pub fn new() -> Result<Self> {
        let dotenv = DotEnv::new("");
        let token = if let Some(token) = dotenv.get_var("INFLUXDB_TOKEN".to_owned()) {
            token
        } else {
            anyhow::bail!("INFLUXDB_TOKEN not found in .env file");
        };
        println!("token {}...", token);
        Ok(Self {
            client: Client::new(INFLUXDB_URL, INFLUXDB_DB).with_token(token),
        })
    }

    pub async fn query<Q>(&self, q: Q) -> Result<String>
    where
        Q: Query,
    {
        match self.client.query(q).await {
            Ok(response) => Ok(response),
            Err(e) => anyhow::bail!("InfluxDB query failed: {}", e),
        }
    }

    /// Checks if a given git_ref exists in db
    pub async fn git_ref_exists(&self, git_ref: &str) -> Result<bool> {
        let query = format!(
            "SELECT COUNT(total_throughput_megabytes_per_second) AS count \
             FROM \"performance_metrics\" \
             WHERE \"git_ref\" = '{}'",
            git_ref
        );

        let read_query = ReadQuery::new(query);
        let read_result = self
            .client
            .query(read_query)
            .await
            .map_err(|e| anyhow::anyhow!("InfluxDB query failed: {}", e))?;

        let response: InfluxResponse = serde_json::from_str(&read_result)?;
        if let Some(series) = response
            .results
            .first()
            .and_then(|r| r.series.as_ref())
            .and_then(|s| s.first())
        {
            if let Some(count_value) = series
                .values
                .first()
                .and_then(|v| v.get(1))
                .and_then(|c| c.as_i64())
            {
                return Ok(count_value > 0);
            }
        }
        Ok(false)
    }
}
