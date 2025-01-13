use crate::config::get_api_base_url;
use crate::error::{IggyDashboardError, Result};
use gloo::console::log;
use gloo::net::http::Request;
use shared::{BenchmarkDetails, BenchmarkHardware, BenchmarkInfoFromDirectoryName, VersionInfo};
use std::sync::atomic::{AtomicBool, Ordering};

static HEALTH_CHECK_DONE: AtomicBool = AtomicBool::new(false);

async fn check_server_health() -> Result<()> {
    if HEALTH_CHECK_DONE.load(Ordering::Relaxed) {
        return Ok(());
    }

    let url = format!("{}/health", get_api_base_url());
    log!(format!("Checking health at: {}", url));

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyDashboardError::HealthCheck(format!("Network error: {}", e)))?;

    if !resp.ok() {
        return Err(IggyDashboardError::HealthCheck(format!(
            "Server returned {}",
            resp.status()
        )));
    }

    HEALTH_CHECK_DONE.store(true, Ordering::Relaxed);
    Ok(())
}

pub async fn fetch_unique_benchmarks(
    version: Option<&str>,
    hardware: Option<&str>,
) -> Result<Vec<BenchmarkInfoFromDirectoryName>> {
    check_server_health().await?;

    let version =
        version.ok_or_else(|| IggyDashboardError::Server("Version is required".into()))?;
    let url = if let Some(hardware) = hardware {
        format!(
            "{}/api/benchmarks/{}/{}",
            get_api_base_url(),
            version,
            hardware
        )
    } else {
        format!("{}/api/benchmarks/{}", get_api_base_url(), version)
    };

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyDashboardError::Network(e.to_string()))?;

    if !resp.ok() {
        return Err(IggyDashboardError::Server(resp.status().to_string()));
    }

    resp.json()
        .await
        .map_err(|e| IggyDashboardError::Parse(e.to_string()))
}

pub async fn fetch_benchmark_info(benchmark_path: &str) -> Result<BenchmarkDetails> {
    check_server_health().await?;

    let url = format!(
        "{}/api/benchmark_info/{}",
        get_api_base_url(),
        benchmark_path
    );

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyDashboardError::Network(e.to_string()))?;

    if !resp.ok() {
        return Err(IggyDashboardError::Server(format!(
            "Failed to fetch benchmark info: {}",
            resp.status()
        )));
    }

    resp.json()
        .await
        .map_err(|e| IggyDashboardError::Parse(e.to_string()))
}

pub async fn fetch_versions_for_hardware(hardware: &str) -> Result<Vec<String>> {
    check_server_health().await?;

    let url = format!("{}/api/versions/{}", get_api_base_url(), hardware);

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyDashboardError::Network(e.to_string()))?;

    if !resp.ok() {
        return Err(IggyDashboardError::Server(resp.status().to_string()));
    }

    let version_info: Vec<VersionInfo> = resp
        .json()
        .await
        .map_err(|e| IggyDashboardError::Parse(e.to_string()))?;

    let versions: Vec<_> = version_info.into_iter().map(|info| info.version).collect();
    log!(format!(
        "Found versions for hardware {}: {:?}",
        hardware, versions
    ));
    Ok(versions)
}

pub async fn fetch_available_hardware() -> Result<Vec<BenchmarkHardware>> {
    check_server_health().await?;

    let url = format!("{}/api/hardware", get_api_base_url());
    log!(format!("Fetching hardware from: {}", url));

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyDashboardError::Network(e.to_string()))?;

    if !resp.ok() {
        return Err(IggyDashboardError::Server(resp.status().to_string()));
    }

    resp.json()
        .await
        .map_err(|e| IggyDashboardError::Parse(e.to_string()))
}
