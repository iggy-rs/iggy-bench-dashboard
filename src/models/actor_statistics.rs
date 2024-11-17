use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BenchmarkActorStatistics {
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
