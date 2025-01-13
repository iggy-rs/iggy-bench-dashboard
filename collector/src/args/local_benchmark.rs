use crate::validate::Validatable;
use anyhow::Result;
use clap::Args;
use std::path::Path;

#[derive(Debug, Args)]
pub struct LocalBenchmarkArgs {
    /// Path to the `iggy` repository
    #[arg(long)]
    pub directory: String,

    /// Git ref (tag, branch or sha1) to start benchmark from
    #[arg(long)]
    pub git_ref: String,

    /// How many commits or tags to go back
    #[arg(long)]
    pub count: u64,
}

impl Validatable for LocalBenchmarkArgs {
    fn validate(&self) -> Result<()> {
        // Check if directory exists
        if !Path::new(&self.directory).exists() {
            anyhow::bail!("Directory '{}' does not exist", self.directory);
        }

        // Check if directory is a github repository
        let git_dir = Path::new(&self.directory).join(".git");
        if !git_dir.exists() {
            anyhow::bail!("Directory '{}' is not a git repository", self.directory);
        }

        Ok(())
    }
}
