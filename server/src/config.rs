use crate::error::IggyDashboardServerError;
use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Parser)]
#[command(author, version, about, long_about = None)]
pub struct IggyDashboardServerConfig {
    /// Server host address
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Server port
    #[arg(long, default_value_t = 8081)]
    pub port: u16,

    /// Directory containing performance results
    #[arg(long, default_value = "./performance_results")]
    pub results_dir: PathBuf,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// Allowed CORS origins (comma-separated)
    #[arg(long, default_value = "*")]
    pub cors_origins: String,

    /// Rate limit per second per IP
    #[arg(long, default_value_t = 50)]
    pub rate_limit: u32,
}

impl IggyDashboardServerConfig {
    pub fn parse() -> Self {
        Self::parse_from(std::env::args())
    }

    pub fn validate(&self) -> Result<(), IggyDashboardServerError> {
        if !self.results_dir.exists() {
            return Err(IggyDashboardServerError::InvalidPath(format!(
                "Results directory does not exist: {}",
                self.results_dir.display()
            )));
        }
        if !self.results_dir.is_dir() {
            return Err(IggyDashboardServerError::InvalidPath(format!(
                "Results path is not a directory: {}",
                self.results_dir.display()
            )));
        }
        Ok(())
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn cors_origins_list(&self) -> Vec<String> {
        self.cors_origins
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }
}
