use crate::error::IggyDashboardServerError;
use actix_web::{get, web, HttpRequest, HttpResponse};
use shared::{
    BenchmarkData, BenchmarkDataJson, BenchmarkDetails, BenchmarkInfoFromDirectoryName,
    BenchmarkTrendData, VersionInfo,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

type Result<T> = std::result::Result<T, IggyDashboardServerError>;

pub struct AppState {
    pub results_dir: PathBuf,
}

fn get_client_ip(req: &HttpRequest) -> String {
    req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string()
}

fn read_dir_entries(path: &Path) -> Result<impl Iterator<Item = Result<std::fs::DirEntry>>> {
    Ok(std::fs::read_dir(path)
        .map_err(|_| IggyDashboardServerError::NotFound("Results directory not found".to_string()))?
        .map(|r| r.map_err(|e| IggyDashboardServerError::DirEntry(e.to_string()))))
}

fn is_dir_entry(entry: &std::fs::DirEntry) -> Result<bool> {
    Ok(entry
        .file_type()
        .map_err(|e| IggyDashboardServerError::DirEntry(e.to_string()))?
        .is_dir())
}

#[get("/health")]
pub async fn health_check(req: HttpRequest) -> Result<HttpResponse> {
    let client_ip = get_client_ip(&req);
    info!("Health check request from {}", client_ip);
    Ok(HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" })))
}

#[get("/api/versions/{hardware}")]
pub async fn list_versions(
    data: web::Data<AppState>,
    hardware: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_ip = get_client_ip(&req);
    let hardware = hardware.into_inner();
    info!(
        "Listing versions for hardware {} from client {}",
        hardware, client_ip
    );

    let mut versions = HashSet::new();

    for entry in read_dir_entries(&data.results_dir)? {
        let entry = entry?;

        if is_dir_entry(&entry)? {
            if let Some(benchmark) =
                BenchmarkInfoFromDirectoryName::from_dirname(&entry.file_name().to_string_lossy())
            {
                if benchmark.hardware == hardware {
                    versions.insert(benchmark.version);
                }
            }
        }
    }

    let mut versions: Vec<_> = versions
        .into_iter()
        .map(|version| VersionInfo { version, count: 0 })
        .collect();
    versions.sort_by(|a, b| b.version.cmp(&a.version));

    debug!(
        "Found {} versions for hardware {} from client {}",
        versions.len(),
        hardware,
        client_ip
    );
    Ok(HttpResponse::Ok().json(versions))
}

#[get("/api/hardware")]
pub async fn list_hardware(data: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse> {
    let client_ip = get_client_ip(&req);
    info!("Listing hardware for client {}", client_ip);

    let mut hardware_set = HashSet::new();
    let mut hardware_details = Vec::new();

    for entry in read_dir_entries(&data.results_dir)? {
        let entry = entry?;

        if is_dir_entry(&entry)? {
            if let Some(benchmark) =
                BenchmarkInfoFromDirectoryName::from_dirname(&entry.file_name().to_string_lossy())
            {
                if !hardware_set.contains(&benchmark.hardware) {
                    // Try to read hardware details from data.json
                    let data_path = data.results_dir.join(format!(
                        "{}_{}_{}/data.json",
                        benchmark.name, benchmark.version, benchmark.hardware
                    ));

                    if let Ok(content) = std::fs::read_to_string(&data_path) {
                        if let Ok(output) = serde_json::from_str::<BenchmarkDetails>(&content) {
                            hardware_details.push(output.hardware);
                            hardware_set.insert(benchmark.hardware);
                        }
                    }
                }
            }
        }
    }

    // Sort by hostname for consistency
    hardware_details.sort_by(|a, b| a.hostname.cmp(&b.hostname));
    // Remove duplicates based on hostname
    hardware_details.dedup_by(|a, b| a.hostname == b.hostname);

    info!(
        "Found {} unique hardware configurations from client {}",
        hardware_details.len(),
        client_ip
    );
    Ok(HttpResponse::Ok().json(hardware_details))
}

#[get("/api/benchmarks/{version}")]
pub async fn list_benchmarks(
    data: web::Data<AppState>,
    version: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_ip = get_client_ip(&req);
    let version = version.into_inner();
    info!(
        "Listing benchmarks for version {} from client {}",
        version, client_ip
    );

    let mut benchmarks = Vec::new();

    for entry in read_dir_entries(&data.results_dir)? {
        let entry = entry?;

        if is_dir_entry(&entry)? {
            if let Some(mut benchmark) =
                BenchmarkInfoFromDirectoryName::from_dirname(&entry.file_name().to_string_lossy())
            {
                if benchmark.version == version {
                    load_pretty_name(&data.results_dir, &mut benchmark)?;
                    benchmarks.push(benchmark);
                }
            }
        }
    }

    benchmarks.sort_by(|a, b| a.name.cmp(&b.name));

    info!(
        "Found {} benchmarks for version {} from client {}",
        benchmarks.len(),
        version,
        client_ip
    );
    Ok(HttpResponse::Ok().json(benchmarks))
}

#[get("/api/benchmarks/{version}/{hardware}")]
pub async fn list_benchmarks_with_hardware(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_ip = get_client_ip(&req);
    let (version, hardware) = path.into_inner();
    info!(
        "Listing benchmarks for version {} and hardware {} from client {}",
        version, hardware, client_ip
    );

    let mut benchmarks = Vec::new();

    for entry in read_dir_entries(&data.results_dir)? {
        let entry = entry?;

        if is_dir_entry(&entry)? {
            if let Some(mut benchmark) =
                BenchmarkInfoFromDirectoryName::from_dirname(&entry.file_name().to_string_lossy())
            {
                if benchmark.version == version && benchmark.hardware == hardware {
                    load_pretty_name(&data.results_dir, &mut benchmark)?;
                    benchmarks.push(benchmark);
                }
            }
        }
    }

    benchmarks.sort_by(|a, b| a.name.cmp(&b.name));

    debug!(
        "Found {} benchmarks for version {} and hardware {} from client {}",
        benchmarks.len(),
        version,
        hardware,
        client_ip
    );
    Ok(HttpResponse::Ok().json(benchmarks))
}

#[get("/api/benchmark_info/{benchmark_path}")]
pub async fn get_benchmark_info(
    data: web::Data<AppState>,
    benchmark_path: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_ip = get_client_ip(&req);
    let benchmark_path = benchmark_path.into_inner();
    info!(
        "Getting benchmark info for {} from client {}",
        benchmark_path, client_ip
    );

    let data_path = data.results_dir.join(&benchmark_path).join("data.json");
    let data_content = std::fs::read_to_string(&data_path).map_err(|_| {
        IggyDashboardServerError::NotFound(format!("Data file not found for {}", benchmark_path))
    })?;

    let benchmark_info: BenchmarkDetails = serde_json::from_str(&data_content)
        .map_err(|e| IggyDashboardServerError::InvalidJson(e.to_string()))?;

    info!(
        "Found benchmark info for {} from client {}",
        benchmark_path, client_ip
    );
    Ok(HttpResponse::Ok().json(benchmark_info))
}

#[get("/api/trend/{benchmark}/{hardware}")]
pub async fn get_benchmark_trend(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let (benchmark, hardware) = path.into_inner();
    let client_ip = get_client_ip(&req);
    info!(
        "Getting trend data for benchmark {} on hardware {} from client {}",
        benchmark, hardware, client_ip
    );

    let mut trend_data = Vec::new();

    for entry in read_dir_entries(&data.results_dir)? {
        let entry = entry?;

        if !is_dir_entry(&entry)? {
            continue;
        }

        if let Some(benchmark_info) = BenchmarkInfoFromDirectoryName::from_dirname(
            entry.file_name().to_string_lossy().as_ref(),
        ) {
            if benchmark_info.name == benchmark && benchmark_info.hardware == hardware {
                let data_path = entry.path().join("data.json");
                if let Ok(data) = std::fs::read_to_string(&data_path) {
                    if let Ok(data_json) = serde_json::from_str::<BenchmarkDataJson>(&data) {
                        trend_data.push(BenchmarkTrendData {
                            version: benchmark_info.version.clone(),
                            data: BenchmarkData {
                                latency_avg: data_json.summary.average_avg_latency_ms,
                                latency_p50: data_json.summary.average_p50_latency_ms,
                                latency_p95: data_json.summary.average_p95_latency_ms,
                                latency_p99: data_json.summary.average_p99_latency_ms,
                                latency_p999: data_json.summary.average_p999_latency_ms,
                                throughput_mb: data_json
                                    .summary
                                    .average_throughput_megabytes_per_second,
                                throughput_msgs: data_json
                                    .summary
                                    .average_throughput_messages_per_second,
                            },
                        });
                    }
                }
            }
        }
    }

    // Sort by version
    trend_data.sort_by(|a, b| a.version.cmp(&b.version));

    Ok(HttpResponse::Ok().json(trend_data))
}

fn load_pretty_name(
    results_dir: &Path,
    benchmark: &mut BenchmarkInfoFromDirectoryName,
) -> Result<()> {
    let data_path = results_dir.join(format!(
        "{}_{}_{}/data.json",
        benchmark.name, benchmark.version, benchmark.hardware
    ));

    if let Ok(content) = std::fs::read_to_string(&data_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(params) = json.get("params") {
                if let Some(pretty_name) = params.get("pretty_name") {
                    if let Some(pretty_name) = pretty_name.as_str() {
                        benchmark.pretty_name = Some(pretty_name.to_string());
                    }
                }
            }
        }
    }
    Ok(())
}
