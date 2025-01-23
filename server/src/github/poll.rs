
async fn poll_github(&self, branch: &str, interval_seconds: u64) -> Result<()> {
    let gh = IggyDashboardGithubClient::new()?;

    info!("Polling GitHub for successful workflow runs on branch {branch} every {interval_seconds} seconds...");

    loop {
        let workflows = gh.get_successful_workflow_runs(branch).await?;
        let tags = gh.get_server_tags().await?;

        for workflow in workflows {
            let sha1 = &workflow.head_sha;

            let gitref =
                if let Some(tag) = IggyDashboardGithubClient::get_tag_for_commit(&tags, sha1) {
                    tag.name
                } else {
                    sha1.clone().chars().take(8).collect()
                };

            info!(
                "Checking if git ref {} sha1 {} exists in db...",
                gitref, sha1
            );

            let workflow_id = workflow.id;
            let artifacts_dir = gh.download_artifact(*workflow_id).await?;
            let dir = get_performance_results_directory(&artifacts_dir).await?;

            // Copy results to the output directory
            tokio::fs::create_dir_all(&self.args.output_dir).await?;
            let target_dir = format!("{}/{}", self.args.output_dir, gitref);
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
