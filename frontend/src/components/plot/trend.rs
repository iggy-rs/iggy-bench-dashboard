use charming::{
    component::{
        Axis, DataView, Feature, Grid, Legend, LegendSelectedMode, Restore, SaveAsImage, Title,
        Toolbox, ToolboxDataZoom,
    },
    element::{AxisType, ItemStyle, LineStyle, NameLocation, Symbol, TextStyle, Tooltip, Trigger},
    series::Line,
    theme::Theme,
    Chart, Echarts, WasmRenderer,
};

use super::{PlotConfig, PlotType, TrendPlotData};
pub fn create_chart(
    config: &PlotConfig,
    plot_data: &TrendPlotData,
    plot_type: &PlotType,
) -> Result<Echarts, String> {
    let chart = match plot_type {
        PlotType::Latency => create_latency_chart(plot_data, config.is_dark),
        PlotType::Throughput => create_throughput_chart(plot_data, config.is_dark),
        PlotType::ThroughputMb => create_throughput_mb_chart(plot_data, config.is_dark),
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

fn create_latency_chart(plot_data: &TrendPlotData, is_dark: bool) -> Chart {
    let latencies: Vec<f64> = plot_data.data.iter().map(|d| d.data.latency_avg).collect();
    let p95_latencies: Vec<f64> = plot_data.data.iter().map(|d| d.data.latency_p95).collect();
    let p99_latencies: Vec<f64> = plot_data.data.iter().map(|d| d.data.latency_p99).collect();
    let p999_latencies: Vec<f64> = plot_data.data.iter().map(|d| d.data.latency_p999).collect();

    Chart::new()
        .background_color(if is_dark { "#242424" } else { "#ffffff" })
        .title(
            Title::new()
                .text("Latency Trend")
                .left("center")
                .top(10)
                .text_style(TextStyle::new().font_size(20).font_weight("bold")),
        )
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(
            Legend::new()
                .bottom("5%")
                .data(vec![
                    "Average Latency",
                    "P95 Latency",
                    "P99 Latency",
                    "P999 Latency",
                ])
                .selected_mode(LegendSelectedMode::Multiple),
        )
        .grid(Grid::new().left("5%").right("5%").top("15%").bottom("15%"))
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_zoom(ToolboxDataZoom::new())
                    .data_view(DataView::new())
                    .restore(Restore::new())
                    .save_as_image(SaveAsImage::new()),
            ),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(plot_data.versions.clone())
                .name("Version")
                .name_location(NameLocation::Center)
                .name_gap(35),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Latency (ms)")
                .name_location(NameLocation::Center)
                .name_gap(45),
        )
        .series(
            Line::new()
                .name("Average Latency")
                .data(latencies)
                .symbol(Symbol::Circle)
                .symbol_size(8.0)
                .line_style(LineStyle::new().width(3.0))
                .item_style(ItemStyle::new().color("#5470c6")),
        )
        .series(
            Line::new()
                .name("P95 Latency")
                .data(p95_latencies)
                .symbol(Symbol::Triangle)
                .symbol_size(8.0)
                .line_style(LineStyle::new().width(3.0))
                .item_style(ItemStyle::new().color("#91cc75")),
        )
        .series(
            Line::new()
                .name("P99 Latency")
                .data(p99_latencies)
                .symbol(Symbol::Diamond)
                .symbol_size(8.0)
                .line_style(LineStyle::new().width(3.0))
                .item_style(ItemStyle::new().color("#fac858")),
        )
        .series(
            Line::new()
                .name("P999 Latency")
                .data(p999_latencies)
                .symbol(Symbol::Rect)
                .symbol_size(8.0)
                .line_style(LineStyle::new().width(3.0))
                .item_style(ItemStyle::new().color("#ee6666")),
        )
}

fn create_throughput_chart(plot_data: &TrendPlotData, is_dark: bool) -> Chart {
    let throughput: Vec<f64> = plot_data
        .data
        .iter()
        .map(|d| d.data.throughput_msgs)
        .collect();

    Chart::new()
        .background_color(if is_dark { "#242424" } else { "#ffffff" })
        .title(
            Title::new()
                .text("Throughput (Messages/s)")
                .left("center")
                .top(10)
                .text_style(TextStyle::new().font_size(20).font_weight("bold")),
        )
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(
            Legend::new()
                .bottom("5%")
                .data(vec!["Producer Throughput", "Consumer Throughput"])
                .selected_mode(LegendSelectedMode::Multiple),
        )
        .grid(Grid::new().left("5%").right("5%").top("15%").bottom("15%"))
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_zoom(ToolboxDataZoom::new())
                    .data_view(DataView::new())
                    .restore(Restore::new())
                    .save_as_image(SaveAsImage::new()),
            ),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(plot_data.versions.clone())
                .name("Version")
                .name_location(NameLocation::Center)
                .name_gap(35),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Messages per second")
                .name_location(NameLocation::Center)
                .name_gap(45),
        )
        .series(
            Line::new()
                .data(throughput)
                .symbol(Symbol::Circle)
                .symbol_size(8.0)
                .line_style(LineStyle::new().width(3.0))
                .item_style(ItemStyle::new().color("#5470c6")),
        )
}

fn create_throughput_mb_chart(plot_data: &TrendPlotData, is_dark: bool) -> Chart {
    let throughput: Vec<f64> = plot_data
        .data
        .iter()
        .map(|d| d.data.throughput_mb)
        .collect();

    Chart::new()
        .background_color(if is_dark { "#242424" } else { "#ffffff" })
        .title(
            Title::new()
                .text("Throughput (MB/s)")
                .left("center")
                .top(10)
                .text_style(TextStyle::new().font_size(20).font_weight("bold")),
        )
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(
            Legend::new()
                .bottom("5%")
                .data(vec!["Producer Throughput", "Consumer Throughput"])
                .selected_mode(LegendSelectedMode::Multiple),
        )
        .grid(Grid::new().left("5%").right("5%").top("15%").bottom("15%"))
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_zoom(ToolboxDataZoom::new())
                    .data_view(DataView::new())
                    .restore(Restore::new())
                    .save_as_image(SaveAsImage::new()),
            ),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(plot_data.versions.clone())
                .name("Version")
                .name_location(NameLocation::Center)
                .name_gap(35),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Megabytes per second")
                .name_location(NameLocation::Center)
                .name_gap(45),
        )
        .series(
            Line::new()
                .data(throughput)
                .symbol(Symbol::Circle)
                .symbol_size(8.0)
                .line_style(LineStyle::new().width(3.0))
                .item_style(ItemStyle::new().color("#5470c6")),
        )
}
