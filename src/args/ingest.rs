use crate::validate::Validatable;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct IngestArgs {
    /// Directory containing performance results to ingest
    #[arg(long, short, group = "ingest_source")]
    pub directory: Option<String>,

    /// GitHub Actions workflow run ID to download artifacts from
    #[arg(long, short, group = "ingest_source")]
    pub workflow_id: Option<u64>,
}

impl Validatable for IngestArgs {
    fn validate(&self) -> Result<()> {
        if let Some(directory) = &self.directory {
            // Check if directory exists
            if !std::path::Path::new(directory).exists() {
                anyhow::bail!("Directory '{}' does not exist", directory);
            }
        }

        Ok(())
    }
}
