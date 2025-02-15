use crate::{BenchmarkGroupMetricsLight, BenchmarkReportLight};
use iggy_bench_report::{benchmark_kind::BenchmarkKind, group_metrics_kind::GroupMetricsKind};

impl BenchmarkReportLight {
    pub fn subtext(&self) -> String {
        let params_text = self.params.format_params();
        let mut stats = Vec::new();

        // First add latency stats
        let latency_stats = self.format_latency_stats();
        if !latency_stats.is_empty() {
            stats.push(latency_stats);
        }

        // Then add throughput stats
        let throughput_stats = self.format_throughput_stats();
        if !throughput_stats.is_empty() {
            stats.push(throughput_stats);
        }

        // For SendAndPoll tests, add total throughput as last line
        if matches!(
            self.params.benchmark_kind,
            BenchmarkKind::PinnedProducerAndConsumer
                | BenchmarkKind::BalancedProducerAndConsumerGroup
        ) {
            if let Some(total) = self.group_metrics.iter().find(|s| {
                s.summary.kind == GroupMetricsKind::ProducersAndConsumers
                    || s.summary.kind == GroupMetricsKind::ProducingConsumers
            }) {
                stats.push(format!(
                    "Total System Throughput: {:.2} MB/s, {:.0} msg/s",
                    total.summary.total_throughput_megabytes_per_second,
                    total.summary.total_throughput_messages_per_second
                ));
            }
        }
        let stats_text = stats.join("\n");

        format!("{params_text}\n{stats_text}")
    }

    fn format_latency_stats(&self) -> String {
        self.group_metrics
            .iter()
            .filter(|s| s.summary.kind != GroupMetricsKind::ProducersAndConsumers) // Skip total summary
            .map(|summary| summary.format_latency())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn format_throughput_stats(&self) -> String {
        self.group_metrics
            .iter()
            .filter(|s| s.summary.kind != GroupMetricsKind::ProducersAndConsumers) // Skip total summary as it will be added separately
            .map(|summary| summary.format_throughput_per_actor_kind())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl BenchmarkGroupMetricsLight {
    fn format_throughput_per_actor_kind(&self) -> String {
        format!(
            "{} Throughput  •  Total: {:.2} MB/s, {:.0} msg/s  •  Avg Per {}: {:.2} MB/s, {:.0} msg/s",
            self.summary.kind,
            self.summary.total_throughput_megabytes_per_second,
            self.summary.total_throughput_messages_per_second,
            self.summary.kind.actor(),
            self.summary.average_throughput_megabytes_per_second,
            self.summary.average_throughput_messages_per_second,
        )
    }

    fn format_latency(&self) -> String {
        format!(
            "{} Latency  •  Avg: {:.2} ms  •  Med: {:.2} ms  •  P95: {:.2} ms  •  P99: {:.2} ms  •  P999: {:.2} ms  •  P9999: {:.2} ms",
            self.summary.kind,
            self.summary.average_latency_ms,
            self.summary.average_median_latency_ms,
            self.summary.average_p95_latency_ms,
            self.summary.average_p99_latency_ms,
            self.summary.average_p999_latency_ms,
            self.summary.average_p9999_latency_ms
        )
    }
}
