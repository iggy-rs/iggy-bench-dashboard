use super::{aggregate_statistics::BenchmarkAggregateStatistics, params::BenchmarkParams};
use chrono::{DateTime, TimeZone, Utc};
use influxdb::InfluxDbWriteable;

#[derive(Debug, InfluxDbWriteable)]
pub struct BenchmarkSummary {
    pub time: DateTime<Utc>,
    #[influxdb(tag)]
    pub git_ref: String,
    #[influxdb(tag)]
    pub benchmark_name: String,
    #[influxdb(tag)]
    pub transport: String,
    #[influxdb(tag)]
    pub messages_per_batch: u32,
    #[influxdb(tag)]
    pub message_batches: u32,
    #[influxdb(tag)]
    pub message_size: u32,
    #[influxdb(tag)]
    pub producers: u32,
    #[influxdb(tag)]
    pub consumers: u32,
    #[influxdb(tag)]
    pub streams: u32,
    #[influxdb(tag)]
    pub partitions: u32,
    #[influxdb(tag)]
    pub number_of_consumer_groups: u32,
    #[influxdb(tag)]
    pub disable_parallel_consumers: bool,
    #[influxdb(tag)]
    pub disable_parallel_producers: bool,
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

impl BenchmarkSummary {
    pub fn new(directory: &str, commit_or_tag: &str) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Reading from directory: {}", directory);

        let params_str = std::fs::read_to_string(format!("{}/params.toml", directory))
            .expect("Failed to read params.toml");
        let params: BenchmarkParams =
            toml::from_str(&params_str).expect("Failed to parse params.toml");

        let producers_file =
            std::fs::exists(format!("{}/producers_summary.toml", directory)).unwrap();
        let consumers_file =
            std::fs::exists(format!("{}/consumers_summary.toml", directory)).unwrap();

        if !producers_file && !consumers_file {
            panic!("Failed to find producers_summary.toml or consumers_summary.toml");
        }

        let stats_file = if producers_file {
            format!("{}/producers_summary.toml", directory)
        } else {
            format!("{}/consumers_summary.toml", directory)
        };

        let stats_str = std::fs::read_to_string(stats_file).unwrap();
        let stats: BenchmarkAggregateStatistics =
            toml::from_str(&stats_str).expect("Failed to parse producers_summary.toml");

        Ok(BenchmarkSummary {
            time: Utc.timestamp_nanos(params.timestamp_micros * 1000),
            git_ref: commit_or_tag.to_owned(),
            benchmark_name: params.benchmark_name,
            transport: params.transport,
            messages_per_batch: params.messages_per_batch,
            message_batches: params.message_batches,
            message_size: params.message_size,
            producers: params.producers,
            consumers: params.consumers,
            streams: params.streams,
            partitions: params.partitions,
            number_of_consumer_groups: params.number_of_consumer_groups,
            disable_parallel_consumers: params.disable_parallel_consumers,
            disable_parallel_producers: params.disable_parallel_producers,
            total_throughput_megabytes_per_second: stats.total_throughput_megabytes_per_second,
            total_throughput_messages_per_second: stats.total_throughput_messages_per_second,
            average_throughput_megabytes_per_second: stats.average_throughput_megabytes_per_second,
            average_throughput_messages_per_second: stats.average_throughput_messages_per_second,
            average_p50_latency_ms: stats.average_p50_latency_ms,
            average_p90_latency_ms: stats.average_p90_latency_ms,
            average_p95_latency_ms: stats.average_p95_latency_ms,
            average_p99_latency_ms: stats.average_p99_latency_ms,
            average_p999_latency_ms: stats.average_p999_latency_ms,
            average_avg_latency_ms: stats.average_avg_latency_ms,
            average_median_latency_ms: stats.average_median_latency_ms,
        })
    }
}
