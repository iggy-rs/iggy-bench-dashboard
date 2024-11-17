mod ingest;
mod local_benchmark;
mod poll_github;

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
    subcommand: IggyDashboardSubcommand,
}

impl Validatable for IggyDashboardArgs {
    fn validate(&self) -> Result<()> {
        self.subcommand.validate()
    }
}

#[derive(Debug, Subcommand)]
enum IggyDashboardSubcommand {
    /// Automatically download the latest successful workflow run artifact from GitHub.
    /// This is blocking and will run forever - useful for hosting on a server.
    PollGithub(PollGithubArgs),

    /// Ingest a single performance result from either directory or GitHub artifact and add data to local InfluxDB
    /// This is a one-time operation and will exit after ingesting the data.
    Ingest(IngestArgs),

    /// Local flow to run benchmarks and ingest results
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
