pub mod ingest;
pub mod local_benchmark;
pub mod poll_github;

use crate::validate::Validatable;
use anyhow::Result;
use clap::{Parser, Subcommand};
use ingest::IngestArgs;
use local_benchmark::LocalBenchmarkArgs;
use poll_github::PollGithubArgs;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, verbatim_doc_comment)]
pub struct IggyDashboardArgs {
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

    /// Ingest a single performance result from local dir or GitHub and put data into the database.
    ///
    /// This is a single-shot operation and will exit after ingesting the data and
    /// adding it to the database.
    Ingest(IngestArgs),

    /// Run benchmarks on local iggy repository and ingest the results into the database.
    ///
    /// This is a single-shot operation and will exit after running the benchmarks n-times.
    LocalBenchmark(LocalBenchmarkArgs),
}

impl Validatable for IggyDashboardSubcommand {
    fn validate(&self) -> Result<()> {
        match self {
            IggyDashboardSubcommand::PollGithub(args) => args.validate(),
            IggyDashboardSubcommand::Ingest(args) => args.validate(),
            IggyDashboardSubcommand::LocalBenchmark(args) => args.validate(),
        }
    }
}
