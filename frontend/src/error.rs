use thiserror::Error;

#[derive(Error, Debug)]
pub enum IggyDashboardError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Server error: {0}")]
    Server(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Health check failed: {0}")]
    HealthCheck(String),
}

pub type Result<T> = std::result::Result<T, IggyDashboardError>;
