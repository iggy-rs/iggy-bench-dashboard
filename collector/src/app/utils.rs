use anyhow::{Context, Result};
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

/// Retrieves the full path to the `performance_results` directory within a `TempDir`.
pub async fn get_performance_results_directory(tempdir: &TempDir) -> Result<String> {
    let temp_path = tempdir.path();

    let mut entries = fs::read_dir(temp_path)
        .await
        .with_context(|| format!("Failed to read directory: {}", temp_path.display()))?;

    let mut subdirs: Vec<PathBuf> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path);
        }
    }

    if subdirs.len() != 1 {
        anyhow::bail!(
            "Expected exactly one subdirectory in '{}', found {}.",
            temp_path.display(),
            subdirs.len()
        );
    }

    let sole_subdir = &subdirs[0];
    let perf_results_dir = sole_subdir.join("performance_results");

    let metadata = fs::metadata(&perf_results_dir).await.with_context(|| {
        format!(
            "Failed to access '{}'. It may not exist.",
            perf_results_dir.display()
        )
    })?;

    if !metadata.is_dir() {
        anyhow::bail!(
            "'performance_results' exists at '{}', but it is not a directory.",
            perf_results_dir.display()
        );
    }

    let perf_results_str = perf_results_dir
        .to_str()
        .with_context(|| format!("Path '{}' is not valid UTF-8.", perf_results_dir.display()))?
        .to_owned();

    Ok(perf_results_str)
}

pub async fn consume_benchmark_results(directory: &str) -> Result<()> {
    println!("Consuming benchmark results from {directory}...");

    // let mut git_ref_dir = fs::read_dir(directory).await?;

    // // Iterate over directories in the given directory (git refs - tags or commit hashes)
    // while let Some(entry) = git_ref_dir.next_entry().await? {
    //     if entry.metadata().await?.is_dir() {
    //         let git_ref = entry.file_name().clone().into_string().unwrap();
    //         println!("Processing git ref {git_ref}...");

    //         let mut benchmark_dir = fs::read_dir(entry.path()).await?;

    //         // Iterate over directories in the given commit directory (benchmarks)
    //         while let Some(entry) = benchmark_dir.next_entry().await? {
    //             println!("Processing benchmark {:?}", entry.file_name());
    //             if entry.metadata().await?.is_dir() {
    //                 let entry_path = entry.path().to_str().unwrap().to_owned();
    //                 let summary = BenchmarkSummary::new(&entry_path, &git_ref).context(format!(
    //                     "Failed to create BenchmarkSummary for {entry_path}"
    //                 ))?;
    //                 let uid = summary.uid;
    //                 println!("UID: {uid}");

    //                 let raw_data = BenchmarkActorRecord::from_file(directory, uid, DB_NAME)
    //                     .context(format!(
    //                         "Failed to create BenchmarkActorRecord for {entry_path} with uid {uid}"
    //                     ))?;

    //                 db.query(summary.into_query(DB_NAME)).await?;
    //                 db.query(raw_data).await?;

    //                 println!("Finished processing benchmark {:?}", entry.file_name());
    //             }
    //         }
    //         println!("Finished processing git ref {git_ref}");
    //     }
    // }

    Ok(())
}
