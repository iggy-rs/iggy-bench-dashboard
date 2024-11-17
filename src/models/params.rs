use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BenchmarkParams {
    pub timestamp_micros: i64,
    pub benchmark_name: String,
    pub transport: String,
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
}
