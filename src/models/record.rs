use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BenchmarkRecord {
    pub elapsed_time_us: u64,
    pub latency_us: u64,
    pub messages: u64,
    pub message_batches: u64,
    pub user_data_bytes: u64,
    pub total_bytes: u64,
}
