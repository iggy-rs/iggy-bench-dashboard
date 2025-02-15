pub mod subtext;
pub mod title;

use iggy_bench_report::{
    group_metrics_summary::BenchmarkGroupMetricsSummary, hardware::BenchmarkHardware,
    individual_metrics_summary::BenchmarkIndividualMetricsSummary, params::BenchmarkParams,
    server_stats::BenchmarkServerStats,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A light version of the benchmark report that doesn't include the time series
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct BenchmarkReportLight {
    pub timestamp: String,
    pub uuid: Uuid,
    pub server_stats: BenchmarkServerStats,
    pub params: BenchmarkParams,
    pub hardware: BenchmarkHardware,
    pub group_metrics: Vec<BenchmarkGroupMetricsLight>,
    pub individual_metrics: Vec<BenchmarkIndividualMetricsLight>,
}

/// Same as BenchmarkGroupMetrics, but without the time series
#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub struct BenchmarkGroupMetricsLight {
    pub summary: BenchmarkGroupMetricsSummary,
}

/// Same as BenchmarkIndividualMetrics, but without the time series
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct BenchmarkIndividualMetricsLight {
    pub summary: BenchmarkIndividualMetricsSummary,
}
