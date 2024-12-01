mod github_poller;
mod ingestor;
mod local_benchmark_runner;
mod utils;

use crate::{
    args::{
        ingest::IngestArgs, local_benchmark::LocalBenchmarkArgs, IggyDashboardArgs,
        IggyDashboardSubcommand,
    },
    db::client::IggyDashboardInfluxDbClient,
    github::client::IggyDashboardGithubClient,
};
use anyhow::Result;
use local_benchmark_runner::LocalBenchmarkRunner;
use std::time::Duration;
use tokio::time::sleep;
use utils::{consume_benchmark_results, get_performance_results_directory};

pub const DB_NAME: &str = "performance_metrics";

pub struct IggyDashboardApp {
    args: IggyDashboardArgs,
    db: IggyDashboardInfluxDbClient,
}

impl IggyDashboardApp {
    pub fn new(args: IggyDashboardArgs) -> Result<Self> {
        let db = IggyDashboardInfluxDbClient::new()?;
        Ok(Self { args, db })
    }

    pub async fn run(&self) -> Result<()> {
        match &self.args.subcommand {
            IggyDashboardSubcommand::PollGithub(args) => {
                self.poll_github(&args.branch, args.interval_seconds).await
            }
            IggyDashboardSubcommand::Ingest(args) => self.ingest(args).await,
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

                let is_present_in_db = self.db.git_ref_exists(&git_ref).await?;
                if is_present_in_db {
                    println!("Git ref {} already exists in db", git_ref);
                    continue;
                } else {
                    println!("Detected new git ref {}, adding to db...", git_ref);
                }

                let workflow_id = workflow.id;
                let artifacts_dir = gh.download_artifact(*workflow_id).await?;

                // TODO fix it
                // artifact is downloaded to temp dir which is a directory in which testcase directories are present
                // ❯ tree -L 2 /tmp/.tmpu3XoR
                // /tmp/.tmpu3XoR
                // └── performance_results_server-0.4.83
                //     ├── poll_8p_1000_1000_1000_tcp
                //     ├── poll_8p_1000_100_10000_tcp
                //     ├── send_8s_1000_1000_1000_tcp
                //     └── send_8s_1000_100_10000_tcp
                // Below function will return the performance_results_* directory path from temporary directory
                let dir = get_performance_results_directory(&artifacts_dir).await?;

                consume_benchmark_results(&dir, &self.db).await?;
            }

            sleep(Duration::from_secs(interval_seconds)).await;
        }
    }

    async fn ingest(&self, args: &IngestArgs) -> Result<()> {
        if let Some(directory) = &args.directory {
            consume_benchmark_results(directory, &self.db).await?;
        } else if let Some(workflow_id) = &args.workflow_id {
            let gh = IggyDashboardGithubClient::new()?;
            let artifacts_dir = gh.download_artifact(*workflow_id).await?;
            let dir = get_performance_results_directory(&artifacts_dir).await?;
            consume_benchmark_results(&dir, &self.db).await?;

            // let mut dir = fs::read_dir(artifacts_dir.path()).await?;
            // while let Some(entry) = dir.next_entry().await? {
            //     if entry.metadata().await?.is_dir() {
            //         println!("Processing: {:?}", entry.path().display());
            //         let entry_path = entry.path().to_str().unwrap().to_owned();
            //         let commit_or_tag = get_git_ref_from_path(&entry_path)?;
            //         let data = BenchmarkSummary::new(&entry_path, &commit_or_tag).unwrap();
            //         self.db.query(data.into_query(DB_NAME)).await?;
            //     }
            // }
        }

        Ok(())
    }

    async fn local_benchmark(&self, args: &LocalBenchmarkArgs) -> Result<()> {
        let repo_path = args.directory.clone();
        let local_benchmark = LocalBenchmarkRunner::new(&repo_path)?;
        local_benchmark.fetch_from_remote()?;
        // local_benchmark.checkout_origin_master()?;
        local_benchmark.build_benchmark_bin().await?;
        local_benchmark.copy_scripts_and_bench_to_temp_dir().await?;
        local_benchmark.checkout_to_git_ref(&args.git_ref)?;

        let commits = local_benchmark.get_last_n_commits(&args.git_ref, args.count)?;

        for commit in commits {
            println!("Processing commit: {}", commit);
            local_benchmark.checkout_to_git_ref(&commit)?;
            local_benchmark.run_benchmark().await?;
        }

        let dir = repo_path + "/performance_results";
        consume_benchmark_results(&dir, &self.db).await?;

        Ok(())
    }
}
