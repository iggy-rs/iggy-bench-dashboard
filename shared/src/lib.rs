use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionInfo {
    pub version: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkInfoFromDirectoryName {
    pub name: String,
    pub version: String,
    pub hardware: String,
    pub pretty_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, Default)]
pub struct BenchmarkInfo {
    pub params: BenchmarkParams,
    pub hardware: BenchmarkHardware,
    pub summary: Option<BenchmarkSummary>,
    pub first_producer_summary: Option<BenchmarkActorSummary>,
    pub first_consumer_summary: Option<BenchmarkActorSummary>,
    pub first_producer_raw_data: Option<Vec<BenchmarkRecord>>,
    pub first_consumer_raw_data: Option<Vec<BenchmarkRecord>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct BenchmarkHardware {
    pub cpu_name: String,
    pub cpu_cores: u32,
    pub cpu_frequency_mhz: u32,
    pub total_memory_kb: u64,
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct BenchmarkParams {
    pub timestamp: String,
    pub benchmark_kind: String,
    pub transport: String,
    pub pretty_name: String,
    pub messages_per_batch: u32,
    pub message_batches: u32,
    pub message_size: u32,
    pub producers: u32,
    pub consumers: u32,
    pub streams: u32,
    pub partitions: u32,
    pub number_of_consumer_groups: u32,
    pub disable_parallel_consumers: bool,
    pub disable_parallel_producers: bool,
    pub git_ref: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BenchmarkDetails {
    pub params: BenchmarkParams,
    pub hardware: BenchmarkHardware,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkSummary {
    pub total_throughput_megabytes_per_second: f64,
    pub total_throughput_messages_per_second: f64,
    pub average_throughput_megabytes_per_second: f64,
    pub average_throughput_messages_per_second: f64,
    pub average_p50_latency_ms: f64,
    pub average_p90_latency_ms: f64,
    pub average_p95_latency_ms: f64,
    pub average_p99_latency_ms: f64,
    pub average_p999_latency_ms: f64,
    pub average_avg_latency_ms: f64,
    pub average_median_latency_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkActorSummary {
    pub total_time_secs: f64,
    pub total_user_data_bytes: u64,
    pub total_bytes: u64,
    pub total_messages: u64,
    pub throughput_megabytes_per_second: f64,
    pub throughput_messages_per_second: f64,
    pub p50_latency_ms: f64,
    pub p90_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub p999_latency_ms: f64,
    pub avg_latency_ms: f64,
    pub median_latency_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkDataJson {
    pub summary: BenchmarkSummary,
    pub params: BenchmarkParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkData {
    pub latency_avg: f64,
    pub latency_p50: f64,
    pub latency_p95: f64,
    pub latency_p99: f64,
    pub latency_p999: f64,
    pub throughput_mb: f64,
    pub throughput_msgs: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkRecord {
    pub elapsed_time_us: u64,
    pub latency_us: u64,
    pub messages: u64,
    pub message_batches: u64,
    pub user_data_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkTrendData {
    pub version: String,
    pub data: BenchmarkData,
}

impl BenchmarkInfoFromDirectoryName {
    pub fn from_dirname(dirname: &str) -> Option<Self> {
        let parts: Vec<&str> = dirname.split('_').collect();
        if parts.len() >= 2 {
            if let Some(version) = parts.get(parts.len() - 2) {
                if version.len() == 8 || version.contains('.') {
                    if let Some(hardware) = parts.last() {
                        let name = parts[..parts.len() - 2].join("_");
                        return Some(BenchmarkInfoFromDirectoryName {
                            name,
                            version: version.to_string(),
                            hardware: hardware.to_string(),
                            pretty_name: None,
                        });
                    }
                }
            }
        }
        None
    }
}
