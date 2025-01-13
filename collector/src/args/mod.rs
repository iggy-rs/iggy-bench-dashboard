pub mod local_benchmark;
pub mod poll_github;

use crate::validate::Validatable;
use anyhow::Result;
use clap::{Parser, Subcommand};
use local_benchmark::LocalBenchmarkArgs;
use poll_github::PollGithubArgs;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, verbatim_doc_comment)]
pub struct IggyDashboardArgs {
    /// Directory where to copy benchmark results
    #[arg(long, short)]
    pub output_dir: String,

    /// Log level (error|warn|info|debug|trace)
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// Subcommand to run
    #[command(subcommand)]
    pub subcommand: IggyDashboardSubcommand,
}

impl Validatable for IggyDashboardArgs {
    fn validate(&self) -> Result<()> {
        self.subcommand.validate()
    }
}

#[derive(Debug, Subcommand)]
pub enum IggyDashboardSubcommand {
    /// Automatically download the latest successful workflow run artifact from GitHub.
    ///
    /// This is blocking and will run forever - useful for hosting on a server.
    PollGithub(PollGithubArgs),

    /// Run benchmarks on local iggy repository.
    ///
    /// This is a single-shot operation and will exit after running the benchmarks n-times.
    LocalBenchmark(LocalBenchmarkArgs),
}

impl Validatable for IggyDashboardSubcommand {
    fn validate(&self) -> Result<()> {
        match self {
            IggyDashboardSubcommand::PollGithub(args) => args.validate(),
            IggyDashboardSubcommand::LocalBenchmark(args) => args.validate(),
        }
    }
}
