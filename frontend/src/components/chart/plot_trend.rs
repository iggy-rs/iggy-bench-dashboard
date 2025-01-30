use super::{PlotConfig, PlotType};
use charming::{
    element::{Symbol, Tooltip, Trigger},
    theme::Theme,
    Chart, Echarts, WasmRenderer,
};
use iggy_benchmark_report::{
    group_metrics_kind::GroupMetricsKind,
    group_metrics_summary::BenchmarkGroupMetricsSummary,
    params::BenchmarkParams,
    plotting::{chart::IggyChart, chart_kind::ChartKind},
};
use shared::BenchmarkReportLight;

fn trend_chart_title(params: &BenchmarkParams, kind: ChartKind) -> String {
    if let Some(remark) = &params.remark {
        format!(
            "{} Trend - {} Benchmark ({})",
            kind, params.benchmark_kind, remark
        )
    } else {
        format!("{} Trend - {} Benchmark", kind, params.benchmark_kind)
    }
}

pub fn create_chart(
    config: &PlotConfig,
    plot_data: &[BenchmarkReportLight],
    plot_type: &PlotType,
) -> Result<Echarts, String> {
    let chart = match plot_type {
        PlotType::Latency => create_latency_trend_chart(plot_data, config.is_dark),
        PlotType::Throughput => create_throughput_trend_chart(plot_data, config.is_dark),
    };

    let renderer = if config.is_dark {
        WasmRenderer::new(config.width, config.height).theme(Theme::Dark)
    } else {
        WasmRenderer::new(config.width, config.height).theme(Theme::Default)
    };

    renderer
        .render(&config.element_id, &chart)
        .map_err(|e| e.to_string())
}

fn create_latency_trend_chart(data: &[BenchmarkReportLight], is_dark: bool) -> Chart {
    let subtext = data[0].params.format_params();
    let title = trend_chart_title(&data[0].params, ChartKind::Latency);

    // Collect all GitRefs for the x-axis
    let gitrefs: Vec<String> = data
        .iter()
        .map(|d| {
            d.params
                .gitref
                .clone()
                .unwrap_or_else(|| "Unknown".to_string())
        })
        .collect();

    // We will accumulate latencies in separate vectors for producers and consumers.
    // Each vector is aligned with the `data` array by index,
    // so the nth element in each vector corresponds to the nth BenchmarkReportLight.
    let mut producer_avg_latencies = Vec::new();
    let mut producer_p95_latencies = Vec::new();
    let mut producer_p99_latencies = Vec::new();
    let mut producer_p999_latencies = Vec::new();

    let mut consumer_avg_latencies = Vec::new();
    let mut consumer_p95_latencies = Vec::new();
    let mut consumer_p99_latencies = Vec::new();
    let mut consumer_p999_latencies = Vec::new();

    let mut chart = IggyChart::new(&title, &subtext, is_dark)
        .with_category_x_axis("Version", gitrefs)
        .with_y_axis("Latency [ms]");

    for report in data {
        let mut send_summary: Option<&BenchmarkGroupMetricsSummary> = None;
        let mut poll_summary: Option<&BenchmarkGroupMetricsSummary> = None;

        for group_metric in &report.group_metrics {
            match group_metric.summary.kind {
                GroupMetricsKind::Producers => {
                    send_summary = Some(&group_metric.summary);
                }
                GroupMetricsKind::Consumers => {
                    poll_summary = Some(&group_metric.summary);
                }
                // GroupMetricsKind::ProducersAndConsumers => {
                // for now ignored
                // }
                _ => {}
            }
        }

        if let Some(summary) = send_summary {
            producer_avg_latencies.push(summary.average_latency_ms);
            producer_p95_latencies.push(summary.average_p95_latency_ms);
            producer_p99_latencies.push(summary.average_p99_latency_ms);
            producer_p999_latencies.push(summary.average_p999_latency_ms);
        }

        if let Some(summary) = poll_summary {
            consumer_avg_latencies.push(summary.average_latency_ms);
            consumer_p95_latencies.push(summary.average_p95_latency_ms);
            consumer_p99_latencies.push(summary.average_p99_latency_ms);
            consumer_p999_latencies.push(summary.average_p999_latency_ms);
        }
    }

    chart = if !producer_avg_latencies.is_empty() {
        chart
            .add_series(
                "Producer Average Latency",
                producer_avg_latencies,
                Symbol::Circle,
                "#5470c6",
            )
            .add_series(
                "Producer P95 Latency",
                producer_p95_latencies,
                Symbol::Triangle,
                "#91cc75",
            )
            .add_series(
                "Producer P99 Latency",
                producer_p99_latencies,
                Symbol::Diamond,
                "#fac858",
            )
            .add_series(
                "Producer P999 Latency",
                producer_p999_latencies,
                Symbol::Rect,
                "#ee6666",
            )
    } else {
        chart
    };

    chart = if !consumer_avg_latencies.is_empty() {
        chart
            .add_series(
                "Consumer Average Latency",
                consumer_avg_latencies,
                Symbol::Circle,
                "#73c0de",
            )
            .add_series(
                "Consumer P95 Latency",
                consumer_p95_latencies,
                Symbol::Triangle,
                "#3ba272",
            )
            .add_series(
                "Consumer P99 Latency",
                consumer_p99_latencies,
                Symbol::Diamond,
                "#fc8452",
            )
            .add_series(
                "Consumer P999 Latency",
                consumer_p999_latencies,
                Symbol::Rect,
                "#ea7ccc",
            )
    } else {
        chart
    };

    chart.inner.tooltip(Tooltip::new().trigger(Trigger::Axis))
}

fn create_throughput_trend_chart(data: &[BenchmarkReportLight], is_dark: bool) -> Chart {
    let throughput_msg: Vec<f64> = data
        .iter()
        .map(|d| {
            d.group_metrics[0]
                .summary
                .average_throughput_messages_per_second
        })
        .collect();
    let throughput_mb: Vec<f64> = data
        .iter()
        .map(|d| {
            d.group_metrics[0]
                .summary
                .average_throughput_megabytes_per_second
        })
        .collect();

    let subtext = data[0].params.format_params();
    let gitrefs = data
        .iter()
        .map(|d| d.params.gitref.clone().unwrap_or("Unknown".to_string()))
        .collect();
    let title = trend_chart_title(&data[0].params, ChartKind::Throughput);

    IggyChart::new(&title, &subtext, is_dark)
        .with_category_x_axis("Version", gitrefs)
        .with_dual_y_axis("Throughput [MB/s]", "Throughput [msg/s]")
        .add_dual_series(
            "Average Throughput [MB/s]",
            throughput_mb,
            Symbol::Circle,
            "#5470c6",
            0,
        )
        .add_dual_series(
            "Average Throughput [msg/s]",
            throughput_msg,
            Symbol::Triangle,
            "#91cc75",
            1,
        )
        .inner
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
}
