mod github_poller;
mod local_benchmark_runner;
mod utils;

use crate::{
    args::{local_benchmark::LocalBenchmarkArgs, IggyDashboardArgs, IggyDashboardSubcommand},
    github::client::IggyDashboardGithubClient,
};
use anyhow::Result;
use local_benchmark_runner::LocalBenchmarkRunner;
use std::time::Duration;
use tokio::time::sleep;
use utils::{consume_benchmark_results, get_performance_results_directory};

pub struct IggyDashboardApp {
    args: IggyDashboardArgs,
}

impl IggyDashboardApp {
    pub fn new(args: IggyDashboardArgs) -> Result<Self> {
        Ok(Self { args })
    }

    pub async fn run(&self) -> Result<()> {
        match &self.args.subcommand {
            IggyDashboardSubcommand::PollGithub(args) => {
                self.poll_github(&args.branch, args.interval_seconds).await
            }
            IggyDashboardSubcommand::LocalBenchmark(args) => self.local_benchmark(args).await,
        }
    }

    async fn poll_github(&self, branch: &str, interval_seconds: u64) -> Result<()> {
        let gh = IggyDashboardGithubClient::new()?;
        println!("Polling GitHub for successful workflow runs on branch {branch} every {interval_seconds} seconds...");

        loop {
            let workflows = gh.get_successful_workflow_runs(branch).await?;
            let tags = gh.get_server_tags().await?;

            for workflow in workflows {
                let sha1 = &workflow.head_sha;

                let git_ref =
                    if let Some(tag) = IggyDashboardGithubClient::get_tag_for_commit(&tags, sha1) {
                        tag.name
                    } else {
                        sha1.clone().chars().take(8).collect()
                    };

                println!(
                    "Checking if git ref {} sha1 {} exists in db...",
                    git_ref, sha1
                );

                let workflow_id = workflow.id;
                let artifacts_dir = gh.download_artifact(*workflow_id).await?;
                let dir = get_performance_results_directory(&artifacts_dir).await?;

                // Copy results to the output directory
                tokio::fs::create_dir_all(&self.args.output_dir).await?;
                let target_dir = format!("{}/{}", self.args.output_dir, git_ref);
                tokio::fs::create_dir_all(&target_dir).await?;

                // Copy all files from dir to target_dir
                for entry in std::fs::read_dir(&dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        let file_name = path.file_name().unwrap();
                        let target_path = format!("{}/{}", target_dir, file_name.to_string_lossy());
                        tokio::fs::copy(path, target_path).await?;
                    }
                }
            }

            sleep(Duration::from_secs(interval_seconds)).await;
        }
    }

    async fn local_benchmark(&self, args: &LocalBenchmarkArgs) -> Result<()> {
        let repo_path = args.directory.clone();
        let local_benchmark = LocalBenchmarkRunner::new(&repo_path)?;
        local_benchmark.fetch_from_remote()?;
        if !args.skip_master_checkout {
            local_benchmark.checkout_origin_master()?;
        }
        local_benchmark.build_benchmark_bin().await?;
        local_benchmark.copy_scripts_and_bench_to_temp_dir().await?;
        local_benchmark.checkout_to_git_ref(&args.git_ref)?;

        let commits = local_benchmark.get_last_n_commits(&args.git_ref, args.count)?;

        for commit in commits {
            println!("Processing commit: {}", commit);
            local_benchmark.checkout_to_git_ref(&commit)?;
            local_benchmark.run_benchmark().await?;
        }

        let source_dir = repo_path + "/performance_results";
        consume_benchmark_results(&source_dir).await?;

        // Copy results to the output directory
        tokio::fs::create_dir_all(&self.args.output_dir).await?;
        let target_dir = format!("{}/{}", self.args.output_dir, args.git_ref);
        tokio::fs::create_dir_all(&target_dir).await?;

        // Copy all files from source_dir to target_dir
        for entry in std::fs::read_dir(&source_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap();
                let target_path = format!("{}/{}", target_dir, file_name.to_string_lossy());
                tokio::fs::copy(path, target_path).await?;
            }
        }

        Ok(())
    }
}
