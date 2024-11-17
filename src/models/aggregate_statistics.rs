use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BenchmarkAggregateStatistics {
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
