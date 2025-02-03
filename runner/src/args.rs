use std::path::Path;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, verbatim_doc_comment)]
pub struct IggyBenchRunnerArgs {
    /// Directory where to copy benchmark results
    #[arg(long, short)]
    pub output_dir: String,

    /// Log level (error|warn|info|debug|trace)
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// Path to the `iggy` repository
    #[arg(long)]
    pub directory: String,

    /// Git ref (tag, branch or sha1) to start benchmark from
    #[arg(long)]
    pub gitref: String,

    /// How many commits or tags to go back
    #[arg(long)]
    pub count: u64,

    /// Skip checking out master branch before running benchmarks
    #[arg(long)]
    pub skip_master_checkout: bool,
}

impl IggyBenchRunnerArgs {
    pub fn validate(&self) -> Result<()> {
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
