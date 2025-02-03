use crate::config::get_api_base_url;
use crate::error::{IggyBenchDashboardError, Result};
use gloo::console::log;
use gloo::net::http::Request;
use iggy_bench_dashboard_shared::BenchmarkReportLight;
use iggy_bench_report::hardware::BenchmarkHardware;
use iggy_bench_report::report::BenchmarkReport;
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;
use web_sys::window;

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
        .map_err(|e| IggyBenchDashboardError::HealthCheck(format!("Network error: {}", e)))?;

    if !resp.ok() {
        return Err(IggyBenchDashboardError::HealthCheck(format!(
            "Server returned {}",
            resp.status()
        )));
    }

    HEALTH_CHECK_DONE.store(true, Ordering::Relaxed);
    Ok(())
}

pub async fn fetch_hardware_configurations() -> Result<Vec<BenchmarkHardware>> {
    check_server_health().await?;

    let url = format!("{}/api/hardware", get_api_base_url());

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyBenchDashboardError::Network(e.to_string()))?;

    if !resp.ok() {
        return Err(IggyBenchDashboardError::Server(format!(
            "Failed to fetch hardware configurations: {}",
            resp.status()
        )));
    }

    resp.json()
        .await
        .map_err(|e| IggyBenchDashboardError::Parse(e.to_string()))
}

pub async fn fetch_gitrefs_for_hardware(hardware: &str) -> Result<Vec<String>> {
    check_server_health().await?;

    let url = format!("{}/api/gitrefs/{}", get_api_base_url(), hardware);

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyBenchDashboardError::Network(e.to_string()))?;

    if !resp.ok() {
        return Err(IggyBenchDashboardError::Server(format!(
            "Failed to fetch git refs: {}",
            resp.status()
        )));
    }

    resp.json()
        .await
        .map_err(|e| IggyBenchDashboardError::Parse(e.to_string()))
}

pub async fn fetch_benchmarks_for_hardware_and_gitref(
    hardware: &str,
    gitref: &str,
) -> Result<Vec<BenchmarkReportLight>> {
    check_server_health().await?;

    let url = format!(
        "{}/api/benchmarks/{}/{}",
        get_api_base_url(),
        hardware,
        gitref
    );

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyBenchDashboardError::Network(e.to_string()))?;

    if !resp.ok() {
        return Err(IggyBenchDashboardError::Server(format!(
            "Failed to fetch benchmarks: {}",
            resp.status()
        )));
    }

    resp.json()
        .await
        .map_err(|e| IggyBenchDashboardError::Parse(e.to_string()))
}

pub async fn fetch_benchmark_report_full(uuid: &Uuid) -> Result<BenchmarkReport> {
    check_server_health().await?;

    let url = format!("{}/api/benchmark/full/{}", get_api_base_url(), uuid);

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyBenchDashboardError::Network(e.to_string()))?;

    if !resp.ok() {
        return Err(IggyBenchDashboardError::Server(format!(
            "Failed to fetch benchmark report: {}",
            resp.status()
        )));
    }

    resp.json()
        .await
        .map_err(|e| IggyBenchDashboardError::Parse(e.to_string()))
}

pub async fn fetch_benchmark_trend(
    hardware: &str,
    params_identifier: &str,
) -> Result<Vec<BenchmarkReportLight>> {
    check_server_health().await?;

    let url = format!(
        "{}/api/benchmark/trend/{}/{}",
        get_api_base_url(),
        hardware,
        params_identifier
    );

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| IggyBenchDashboardError::Network(e.to_string()))?;

    if !resp.ok() {
        return Err(IggyBenchDashboardError::Server(format!(
            "Failed to fetch benchmark trend: {}",
            resp.status()
        )));
    }

    resp.json()
        .await
        .map_err(|e| IggyBenchDashboardError::Parse(e.to_string()))
}

pub fn download_test_artifacts(uuid: &Uuid) {
    // Create the download URL
    let url = format!("{}/api/artifacts/{}", get_api_base_url(), uuid);

    // Use browser's native download functionality
    if let Some(window) = window() {
        let _ = window
            .location()
            .set_href(&url)
            .map_err(|_| log!("Failed to initiate download"));
    }
}
