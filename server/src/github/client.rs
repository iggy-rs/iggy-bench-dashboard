use anyhow::{Context, Result};
use octocrab::{
    models::{
        repos::Tag,
        workflows::{Run, WorkflowListArtifact},
        RunId,
    },
    params::actions::ArchiveFormat,
    Octocrab,
};
use std::io::Cursor;
use tempfile::TempDir;
use tokio::fs::read_dir;
use tracing::info;
use zip::ZipArchive;

const OWNER: &str = "iggy-rs";
const REPO: &str = "iggy";
const WORKFLOW_FILE: &str = "performance.yml";

pub struct IggyBenchDashboardGithubClient {
    octocrab: Octocrab,
}

impl IggyBenchDashboardGithubClient {
    pub fn new() -> Result<Self> {
        let token = std::env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not set")?;
        let octocrab = Octocrab::builder().personal_token(token).build()?;
        Ok(Self { octocrab })
    }

    /// Download the `performance_results.zip` for a given workflow ID in
    /// `performance.yml` workflows and return the output directory.
    pub async fn download_artifact(&self, workflow_id: u64) -> Result<TempDir> {
        // Fetch the list of workflow runs for "performance.yml" with a successful status
        let runs = self.get_all_workflow_runs().await?;
        if runs.is_empty() {
            anyhow::bail!("No successful workflow runs found")
        }

        let run_id = RunId(workflow_id);

        // Find the specific run with the provided workflow_id
        runs.iter().find(|run| run.id == run_id).context(format!(
            "Workflow run {workflow_id} not found in {WORKFLOW_FILE}"
        ))?;

        // Fetch the list of artifacts for the specific run
        let artifacts = self.get_artifacts_for_workflow_run(run_id).await?;

        // Assuming you want the first artifact; adjust if necessary
        let artifact = &artifacts[0];
        let artifact_id = artifact.id;

        info!("Downloading artifact ID: {}", artifact_id);

        // Download the artifact as bytes in ZIP format
        let bytes = self
            .octocrab
            .actions()
            .download_artifact(OWNER, REPO, artifact_id, ArchiveFormat::Zip)
            .await?;
        info!(
            "Downloaded artifact ID: {}, bytes length: {}",
            artifact_id,
            bytes.len()
        );

        // Create a temporary directory
        let temp_dir = TempDir::new()?;
        let output_dir = temp_dir.path();

        info!("Unzipping to directory: {:?}", output_dir);

        // Unzip the downloaded bytes into the temporary directory
        let cursor = Cursor::new(bytes);
        let mut zip = ZipArchive::new(cursor)?;

        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let outpath = output_dir.join(file.mangled_name());

            if file.is_dir() {
                // It's a directory; create it
                std::fs::create_dir_all(&outpath)?;
            } else {
                // It's a file; ensure the parent directory exists
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                // Create and write the file
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }

            // Optionally, set Unix permissions if applicable
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
                }
            }
        }

        info!("Artifact unzipped to: {:?}", output_dir);

        // After unzipping, asynchronously read and filter directory entries
        let mut entries = Vec::new();
        let mut dir = read_dir(output_dir).await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                entries.push(path);
            }
        }

        // Check if there's exactly one directory
        if entries.len() != 1 {
            anyhow::bail!(
                "Expected exactly one directory in the unzipped artifact directory {}, found {}",
                temp_dir.path().display(),
                entries.len()
            )
        }

        Ok(temp_dir)
    }

    /// Retrieves workflow runs for the specified workflow file (`performance.yml`)
    /// that have a successful status.
    async fn get_all_workflow_runs(&self) -> Result<Vec<Run>> {
        let runs: Vec<Run> = self
            .octocrab
            .workflows(OWNER, REPO)
            .list_runs(WORKFLOW_FILE)
            .status("success")
            .per_page(100)
            .send()
            .await?
            .into_iter()
            .collect();
        Ok(runs)
    }

    /// Retrieves workflow runs for the specified workflow file (`performance.yml`)
    /// that were triggered on given branch and have a successful status.
    pub async fn get_successful_workflow_runs(&self, branch: &str) -> Result<Vec<Run>> {
        let runs: Vec<Run> = self
            .octocrab
            .workflows(OWNER, REPO)
            .list_runs(WORKFLOW_FILE)
            .status("success")
            .branch(branch)
            .send()
            .await?
            .into_iter()
            .collect();

        info!(
            "Found {} successful workflow runs on {branch} branch",
            runs.len(),
        );
        Ok(runs)
    }

    async fn get_artifacts_for_workflow_run(
        &self,
        run_id: RunId,
    ) -> Result<Vec<WorkflowListArtifact>> {
        let artifacts: Vec<WorkflowListArtifact> = self
            .octocrab
            .actions()
            .list_workflow_run_artifacts(OWNER, REPO, run_id)
            .send()
            .await?
            .value
            .context(format!("No artifacts found for workflow run {run_id}"))?
            .into_iter()
            .collect();
        Ok(artifacts)
    }

    pub fn get_tag_for_commit(tags: &Vec<Tag>, commit_sha: &str) -> Option<Tag> {
        for tag in tags {
            if tag.commit.sha.starts_with(commit_sha) {
                return Some(tag.clone());
            }
        }

        None
    }

    pub async fn get_server_tags(&self) -> Result<Vec<Tag>> {
        let mut tags = Vec::new();
        let mut page = self.octocrab.repos(OWNER, REPO).list_tags().send().await?;

        loop {
            for tag in &page {
                if tag.name.starts_with("server") {
                    tags.push(tag.clone());
                }
            }
            page = match self.octocrab.get_page(&page.next).await? {
                Some(next_page) => next_page,
                None => break,
            };
        }

        Ok(tags)
    }
}
