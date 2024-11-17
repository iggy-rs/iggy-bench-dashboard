use crate::validate::Validatable;
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct PollGithubArgs {
    /// How often to poll GitHub for new artifacts, in seconds
    #[arg(short, default_value = "60")]
    interval_seconds: u64,

    /// Branch to filter artifacts by
    #[arg(default_value = "master")]
    branch: String,
}

impl Validatable for PollGithubArgs {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
