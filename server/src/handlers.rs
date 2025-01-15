use crate::cache::BenchmarkCache;
use crate::error::IggyDashboardServerError;
use actix_web::{get, web, HttpRequest, HttpResponse};
use chrono::DateTime;
use shared::{
    BenchmarkData, BenchmarkDataJson, BenchmarkInfo, BenchmarkInfoFromDirectoryName,
    BenchmarkTrendData,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::info;

type Result<T> = std::result::Result<T, IggyDashboardServerError>;

pub struct AppState {
    pub results_dir: PathBuf,
    pub cache: Arc<BenchmarkCache>,
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

    let benchmarks = data.cache.get_hardware_benchmarks(&hardware);
    let mut versions = Vec::new();
    let mut seen_versions = std::collections::HashSet::new();

    for path in benchmarks {
        if let Some(benchmark) = BenchmarkInfoFromDirectoryName::from_dirname(&path) {
            if let Some(details) = data.cache.get_benchmark(&path) {
                info!(
                    "Processing benchmark path: {}, version: {}, git_ref_date: {}",
                    path, benchmark.version, details.params.git_ref_date
                );

                // Only add version if we haven't seen it before
                if seen_versions.insert(benchmark.version.clone()) {
                    versions.push((
                        DateTime::parse_from_str(
                            &details.params.git_ref_date,
                            "%Y-%m-%dT%H:%M:%S%z",
                        )
                        .unwrap_or_else(|_| {
                            DateTime::parse_from_str(
                                "1970-01-01T00:00:00+0000",
                                "%Y-%m-%dT%H:%M:%S%z",
                            )
                            .unwrap()
                        }),
                        benchmark.version,
                    ));
                }
            }
        }
    }

    versions.sort_by(|a, b| b.0.cmp(&a.0));

    let versions = versions
        .into_iter()
        .map(|(_, version)| version)
        .collect::<Vec<String>>();

    info!(
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
    info!("Listing hardware from client {}", client_ip);

    let hardware_list = data.cache.get_hardware_configurations();

    info!(
        "Found {} hardware configurations from client {}",
        hardware_list.len(),
        client_ip
    );
    Ok(HttpResponse::Ok().json(hardware_list))
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

    let benchmarks = data.cache.get_benchmarks_for_version(&version);

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

    let benchmarks = data
        .cache
        .get_benchmarks_for_version_and_hardware(&version, &hardware);

    info!(
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

    let details = data.cache.get_benchmark(&benchmark_path).ok_or_else(|| {
        IggyDashboardServerError::NotFound(format!("Benchmark not found: {}", benchmark_path))
    })?;

    info!(
        "Found benchmark info for {} from client {}",
        benchmark_path, client_ip
    );
    Ok(HttpResponse::Ok().json(details))
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
                        trend_data.push((
                            data_json.params.timestamp,
                            BenchmarkTrendData {
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
                            },
                        ));
                    }
                }
            }
        }
    }

    // Sort by timestamp
    trend_data.sort_by(|a, b| a.0.cmp(&b.0));

    // Return only the BenchmarkTrendData part
    Ok(HttpResponse::Ok().json(
        trend_data
            .into_iter()
            .map(|(_, data)| data)
            .collect::<Vec<_>>(),
    ))
}

#[get("/api/single/{benchmark_path}")]
pub async fn get_single_benchmark(
    data: web::Data<AppState>,
    benchmark_path: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_ip = get_client_ip(&req);
    let benchmark_path = benchmark_path.into_inner();

    info!(
        "Getting single benchmark {} from client {}",
        benchmark_path, client_ip
    );

    let path = data.results_dir.join(&benchmark_path);

    if !path.exists() {
        return Err(IggyDashboardServerError::NotFound(format!(
            "Benchmark {} not found",
            benchmark_path
        )));
    }

    let benchmark_info = load_benchmark_info(&path)?;
    Ok(HttpResponse::Ok().json(benchmark_info))
}

fn load_benchmark_info(path: &Path) -> Result<BenchmarkInfo> {
    let data_path = path.join("data.json");
    let data_content = std::fs::read_to_string(&data_path).map_err(|_| {
        IggyDashboardServerError::NotFound(format!("Data file not found for {}", path.display()))
    })?;

    serde_json::from_str(&data_content)
        .map_err(|e| IggyDashboardServerError::InvalidJson(e.to_string()))
}
