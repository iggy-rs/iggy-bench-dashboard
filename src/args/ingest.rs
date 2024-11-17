use crate::validate::Validatable;
use anyhow::{Context, Result};
use clap::Args;

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct IngestArgs {
    /// Directory containing performance results to ingest
    #[arg(long, short, group = "ingest_source")]
    directory: Option<String>,

    /// GitHub Actions workflow run ID to download artifacts from
    #[arg(long, short, group = "ingest_source")]
    workflow_id: Option<u64>,
}

impl Validatable for IngestArgs {
    fn validate(&self) -> Result<()> {
        if let Some(directory) = &self.directory {
            // Check if directory exists
            if !std::path::Path::new(directory).exists() {
                anyhow::bail!("Directory '{}' does not exist", directory);
            }

            // Check for naming schema: performance_results_{commit or tag}
            let commit_or_tag = directory
                .split("performance_results_")
                .nth(1)
                .context("Path does not contain 'performance_results_'")?
                .split('/')
                .next()
                .context("Path does not contain '/' after 'performance_results_'")?;

            if commit_or_tag.len() != 7 {
                anyhow::bail!("Commit or tag '{}' is not 7 characters long", commit_or_tag);
            }
        }

        Ok(())
    }
}
