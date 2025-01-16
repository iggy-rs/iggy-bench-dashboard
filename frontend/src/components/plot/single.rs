use super::{PlotConfig, PlotType};
use charming::component::LegendSelectedMode;
use charming::element::{ItemStyle, SplitLine, TextAlign, TextStyle};
use charming::theme::Theme;
use charming::{
    component::{
        Axis, DataView, Feature, Grid, Legend, Restore, SaveAsImage, Title, Toolbox,
        ToolboxDataZoom,
    },
    element::{
        AxisLabel, AxisPointer, AxisPointerType, AxisType, LineStyle, NameLocation, Symbol,
        Tooltip, Trigger,
    },
    series::Line,
    Chart,
};
use charming::{Echarts, WasmRenderer};
use gloo::console::log;
use shared::{BenchmarkActorSummary, BenchmarkInfo, BenchmarkRecord};

fn format_throughput_stats(stats: &BenchmarkActorSummary, title_prefix: &str) -> String {
    format!(
        "Average throughput per {}: {:.3} msg/s",
        if title_prefix == "Producer" {
            "producer"
        } else {
            "consumer"
        },
        stats.throughput_messages_per_second,
    )
}

fn format_latency_stats(stats: &BenchmarkActorSummary) -> String {
    format!(
        "Average: {:.3} ms, Median: {:.3} ms, P99: {:.3} ms, P999: {:.3} ms",
        stats.avg_latency_ms, stats.median_latency_ms, stats.p99_latency_ms, stats.p999_latency_ms,
    )
}

fn format_throughput_mb_stats(stats: &BenchmarkActorSummary, title_prefix: &str) -> String {
    format!(
        "Average throughput per {}: {:.3} MB/s",
        if title_prefix == "Producer" {
            "producer"
        } else {
            "consumer"
        },
        stats.throughput_megabytes_per_second,
    )
}

pub fn create_chart(
    config: &PlotConfig,
    data: &BenchmarkInfo,
    plot_type: &PlotType,
) -> Result<Echarts, String> {
    log!(format!(
        "Creating chart for benchmark {}",
        data.params.pretty_name
    ));

    let actors_info = match (data.params.producers, data.params.consumers) {
        (0, c) => format!("{} consumers", c),
        (p, 0) => format!("{} producers", p),
        (p, c) => format!("{} producers/{} consumers", p, c),
    };

    let mut subtext = format!(
        "{}, {} msg/batch, {} batches, {} bytes/msg",
        actors_info,
        data.params.messages_per_batch,
        data.params.message_batches,
        data.params.message_size
    );

    if let Some(overall_stats) = &data.summary {
        subtext = format!(
            "{}\nTotal throughput: {:.2} MB/s, {:.0} messages/s",
            subtext,
            overall_stats.total_throughput_megabytes_per_second,
            overall_stats.total_throughput_messages_per_second
        );
    }

    let chart = if let Some(producer_data) = &data.first_producer_raw_data {
        if let Some(producer_stats) = &data.first_producer_summary {
            match plot_type {
                PlotType::Throughput => plot_throughput_over_time(
                    producer_data,
                    producer_stats,
                    "Producer",
                    &subtext,
                    config.is_dark,
                ),
                PlotType::ThroughputMb => plot_throughput_mb_over_time(
                    producer_data,
                    producer_stats,
                    "Producer",
                    &subtext,
                    config.is_dark,
                ),
                PlotType::Latency => plot_latency_over_time(
                    producer_data,
                    producer_stats,
                    "Producer",
                    &subtext,
                    config.is_dark,
                ),
            }
        } else {
            return Err("No producer stats available".to_string());
        }
    } else if let Some(consumer_data) = &data.first_consumer_raw_data {
        if let Some(consumer_stats) = &data.first_consumer_summary {
            match plot_type {
                PlotType::Throughput => plot_throughput_over_time(
                    consumer_data,
                    consumer_stats,
                    "Consumer",
                    &subtext,
                    config.is_dark,
                ),
                PlotType::ThroughputMb => plot_throughput_mb_over_time(
                    consumer_data,
                    consumer_stats,
                    "Consumer",
                    &subtext,
                    config.is_dark,
                ),
                PlotType::Latency => plot_latency_over_time(
                    consumer_data,
                    consumer_stats,
                    "Consumer",
                    &subtext,
                    config.is_dark,
                ),
            }
        } else {
            return Err("No consumer stats available".to_string());
        }
    } else {
        return Err("No data available to plot".to_string());
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

fn plot_throughput_over_time(
    data: &[BenchmarkRecord],
    stats: &BenchmarkActorSummary,
    title_prefix: &str,
    subtext: &str,
    is_dark: bool,
) -> Chart {
    let mut time_to_values: std::collections::BTreeMap<i64, Vec<f64>> =
        std::collections::BTreeMap::new();

    data.windows(2).for_each(|w| {
        let time_diff = (w[1].elapsed_time_us - w[0].elapsed_time_us) as f64 / 1_000_000.0;
        let messages_diff = (w[1].messages - w[0].messages) as f64;
        let throughput = ((messages_diff / time_diff) * 1000.0).round() / 1000.0;
        let time_key = (w[1].elapsed_time_us / 10_000) as i64; // Round to 0.001s
        time_to_values.entry(time_key).or_default().push(throughput);
    });

    let points: Vec<_> = time_to_values
        .into_iter()
        .map(|(time_key, values)| {
            let avg = (values.iter().sum::<f64>() / values.len() as f64 * 1000.0).round() / 1000.0;
            vec![(time_key as f64 / 100.0), avg]
        })
        .collect();

    let window_size = 50;
    let half_window = window_size / 2;
    let throughputs: Vec<f64> = points.iter().map(|p| p[1]).collect();
    let mut smoothed_throughputs = vec![0.0; throughputs.len()];

    (0..throughputs.len()).for_each(|i| {
        let start = i.saturating_sub(half_window);
        let end = if i + half_window >= throughputs.len() {
            throughputs.len()
        } else {
            i + half_window + 1
        };
        let window = &throughputs[start..end];
        smoothed_throughputs[i] =
            (window.iter().sum::<f64>() / window.len() as f64 * 1000.0).round() / 1000.0;
    });

    let smoothed_points: Vec<Vec<f64>> = points
        .iter()
        .zip(smoothed_throughputs)
        .map(|(p, s)| vec![p[0], s])
        .collect();

    let title = format!("{} Throughput Over Time", title_prefix);
    let stats_text = format_throughput_stats(stats, title_prefix);
    let full_subtext = format!("{}\n{}", subtext, stats_text);

    Chart::new()
        .background_color(if is_dark { "#242424" } else { "#ffffff" })
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Cross)),
        )
        .legend(
            Legend::new()
                .show(true)
                .bottom("5%")
                .selected_mode(LegendSelectedMode::Multiple),
        )
        .grid(Grid::new().left("12%").right("8%").top("15%").bottom("10%"))
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_zoom(ToolboxDataZoom::new())
                    .data_view(DataView::new())
                    .restore(Restore::new())
                    .save_as_image(SaveAsImage::new()),
            ),
        )
        .title(
            Title::new()
                .text(title.clone())
                .subtext(full_subtext)
                .text_align(TextAlign::Center)
                .subtext_style(TextStyle::new().font_size(14))
                .padding(25)
                .item_gap(8)
                .left("50%")
                .top("5%"),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Time (seconds)")
                .name_location(NameLocation::Center)
                .name_gap(35)
                .axis_label(AxisLabel::new().formatter("{value} s"))
                .split_line(SplitLine::new().show(true)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Msg/s")
                .name_location(NameLocation::End)
                .name_gap(15)
                .name_rotation(0)
                .position("left")
                .axis_label(AxisLabel::new().formatter("{value} msg/s"))
                .split_line(SplitLine::new().show(true)),
        )
        .series(
            Line::new()
                .name("Throughput")
                .data(points)
                .symbol(Symbol::None)
                .line_style(LineStyle::new().width(3.0).opacity(0.3))
                .item_style(ItemStyle::new().color("#5470c6")),
        )
        .series(
            Line::new()
                .name("Smoothed Throughput")
                .data(smoothed_points)
                .symbol(Symbol::None)
                .line_style(LineStyle::new().width(3.0))
                .item_style(ItemStyle::new().color("#91cc75")),
        )
}

fn plot_latency_over_time(
    data: &[BenchmarkRecord],
    stats: &BenchmarkActorSummary,
    title_prefix: &str,
    subtext: &str,
    is_dark: bool,
) -> Chart {
    let mut time_to_values: std::collections::BTreeMap<i64, Vec<f64>> =
        std::collections::BTreeMap::new();

    data.windows(2).for_each(|w| {
        let latency = (w[1].latency_us as f64 / 1_000.0 * 1000.0).round() / 1000.0;
        let time_key = (w[1].elapsed_time_us / 10_000) as i64; // Round to 0.001s
        time_to_values.entry(time_key).or_default().push(latency);
    });

    let points: Vec<_> = time_to_values
        .into_iter()
        .map(|(time_key, values)| {
            let avg = (values.iter().sum::<f64>() / values.len() as f64 * 1000.0).round() / 1000.0;
            vec![(time_key as f64 / 100.0), avg]
        })
        .collect();

    let window_size = 50;
    let half_window = window_size / 2;
    let latencies: Vec<f64> = points.iter().map(|p| p[1]).collect();
    let mut smoothed_latencies = vec![0.0; latencies.len()];

    (0..latencies.len()).for_each(|i| {
        let start = i.saturating_sub(half_window);
        let end = if i + half_window >= latencies.len() {
            latencies.len()
        } else {
            i + half_window + 1
        };
        let window = &latencies[start..end];
        smoothed_latencies[i] =
            (window.iter().sum::<f64>() / window.len() as f64 * 1000.0).round() / 1000.0;
    });

    let smoothed_points: Vec<Vec<f64>> = points
        .iter()
        .zip(smoothed_latencies)
        .map(|(p, s)| vec![p[0], s])
        .collect();

    let title = format!("{} Latency Over Time", title_prefix);
    let stats_text = format_latency_stats(stats);
    let full_subtext = format!("{}\n{}", subtext, stats_text);

    Chart::new()
        .background_color(if is_dark { "#242424" } else { "#ffffff" })
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Cross)),
        )
        .legend(
            Legend::new()
                .show(true)
                .bottom("5%")
                .selected_mode(LegendSelectedMode::Multiple),
        )
        .grid(Grid::new().left("12%").right("8%").top("15%").bottom("10%"))
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_zoom(ToolboxDataZoom::new())
                    .data_view(DataView::new())
                    .restore(Restore::new())
                    .save_as_image(SaveAsImage::new()),
            ),
        )
        .title(
            Title::new()
                .text(title.clone())
                .subtext(full_subtext)
                .text_align(TextAlign::Center)
                .subtext_style(TextStyle::new().font_size(14))
                .padding(25)
                .item_gap(8)
                .left("50%")
                .top("5%"),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Time (seconds)")
                .name_location(NameLocation::Center)
                .name_gap(35)
                .axis_label(AxisLabel::new().formatter("{value} s"))
                .split_line(SplitLine::new().show(true)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Latency (ms)")
                .name_location(NameLocation::End)
                .name_gap(15)
                .name_rotation(0)
                .position("left")
                .axis_label(AxisLabel::new().formatter("{value} ms"))
                .split_line(SplitLine::new().show(true)),
        )
        .series(
            Line::new()
                .name("Latency")
                .data(points)
                .symbol(Symbol::None)
                .line_style(LineStyle::new().width(3.0).opacity(0.3))
                .item_style(ItemStyle::new().color("#5470c6")),
        )
        .series(
            Line::new()
                .name("Smoothed Latency")
                .data(smoothed_points)
                .symbol(Symbol::None)
                .line_style(LineStyle::new().width(3.0))
                .item_style(ItemStyle::new().color("#91cc75")),
        )
}

fn plot_throughput_mb_over_time(
    data: &[BenchmarkRecord],
    stats: &BenchmarkActorSummary,
    title_prefix: &str,
    subtext: &str,
    is_dark: bool,
) -> Chart {
    let mut time_to_values: std::collections::BTreeMap<i64, Vec<f64>> =
        std::collections::BTreeMap::new();

    data.windows(2).for_each(|w| {
        let time_diff = (w[1].elapsed_time_us - w[0].elapsed_time_us) as f64 / 1_000_000.0;
        let bytes_diff = (w[1].total_bytes - w[0].total_bytes) as f64;
        let throughput = ((bytes_diff / (1024.0 * 1024.0) / time_diff) * 1000.0).round() / 1000.0;
        let time_key = (w[1].elapsed_time_us / 10_000) as i64; // Round to 0.001s
        time_to_values.entry(time_key).or_default().push(throughput);
    });

    let points: Vec<_> = time_to_values
        .into_iter()
        .map(|(time_key, values)| {
            let avg = (values.iter().sum::<f64>() / values.len() as f64 * 1000.0).round() / 1000.0;
            vec![(time_key as f64 / 100.0), avg]
        })
        .collect();

    let window_size = 50;
    let half_window = window_size / 2;
    let throughputs: Vec<f64> = points.iter().map(|p| p[1]).collect();
    let mut smoothed_throughputs = vec![0.0; throughputs.len()];

    (0..throughputs.len()).for_each(|i| {
        let start = i.saturating_sub(half_window);
        let end = if i + half_window >= throughputs.len() {
            throughputs.len()
        } else {
            i + half_window + 1
        };
        let window = &throughputs[start..end];
        smoothed_throughputs[i] =
            (window.iter().sum::<f64>() / window.len() as f64 * 1000.0).round() / 1000.0;
    });

    let smoothed_points: Vec<Vec<f64>> = points
        .iter()
        .zip(smoothed_throughputs)
        .map(|(p, s)| vec![p[0], s])
        .collect();

    let title = format!("{} Throughput Over Time", title_prefix);
    let stats_text = format_throughput_mb_stats(stats, title_prefix);
    let full_subtext = format!("{}\n{}", subtext, stats_text);

    Chart::new()
        .background_color(if is_dark { "#242424" } else { "#ffffff" })
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Cross)),
        )
        .legend(
            Legend::new()
                .show(true)
                .bottom("5%")
                .selected_mode(LegendSelectedMode::Multiple),
        )
        .grid(Grid::new().left("12%").right("8%").top("15%").bottom("10%"))
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_zoom(ToolboxDataZoom::new())
                    .data_view(DataView::new())
                    .restore(Restore::new())
                    .save_as_image(SaveAsImage::new()),
            ),
        )
        .title(
            Title::new()
                .text(title.clone())
                .subtext(full_subtext)
                .text_align(TextAlign::Center)
                .subtext_style(TextStyle::new().font_size(14))
                .padding(25)
                .item_gap(8)
                .left("50%")
                .top("5%"),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Time (seconds)")
                .name_location(NameLocation::Center)
                .name_gap(35)
                .axis_label(AxisLabel::new().formatter("{value} s"))
                .split_line(SplitLine::new().show(true)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Msg/s")
                .name_location(NameLocation::End)
                .name_gap(15)
                .name_rotation(0)
                .position("left")
                .axis_label(AxisLabel::new().formatter("{value} MB/s"))
                .split_line(SplitLine::new().show(true)),
        )
        .series(
            Line::new()
                .name("Throughput")
                .data(points)
                .symbol(Symbol::None)
                .line_style(LineStyle::new().width(3.0).opacity(0.3))
                .item_style(ItemStyle::new().color("#5470c6")),
        )
        .series(
            Line::new()
                .name("Smoothed Throughput")
                .data(smoothed_points)
                .symbol(Symbol::None)
                .line_style(LineStyle::new().width(3.0))
                .item_style(ItemStyle::new().color("#91cc75")),
        )
}
