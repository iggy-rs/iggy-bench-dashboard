use crate::state::hardware::use_hardware;
use crate::types::MeasurementType;
use charming::{
    component::{Axis, Grid, Legend, LegendSelectedMode, Title},
    element::{AxisType, ItemStyle, LineStyle, NameLocation, Symbol, TextStyle, Tooltip, Trigger},
    series::Line,
    theme::Theme,
    Chart, Echarts, WasmRenderer,
};
use gloo::console::log;
use gloo::net::http::Request;
use gloo::net::Error;
use shared::BenchmarkTrendData;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::use_size;

type CleanupFn = Box<dyn FnOnce()>;

#[derive(Properties, PartialEq)]
pub struct TrendChartProps {
    pub benchmark_name: String,
    pub measurement_type: MeasurementType,
    pub is_dark: bool,
}

async fn fetch_trend_data(
    benchmark: &str,
    hardware: &str,
) -> Result<Vec<BenchmarkTrendData>, Error> {
    let url = format!("/api/trend/{}/{}", benchmark, hardware);
    let resp = Request::get(&url).send().await?;
    resp.json().await
}

#[wasm_bindgen]
extern "C" {
    type EChartsInstance;

    #[wasm_bindgen(js_namespace = echarts)]
    fn getInstanceByDom(element: &web_sys::Element) -> Option<EChartsInstance>;

    #[wasm_bindgen(method)]
    fn dispose(this: &EChartsInstance);
}

#[function_component(TrendChart)]
pub fn trend_chart(props: &TrendChartProps) -> Html {
    let hardware_ctx = use_hardware();
    let chart_data = use_state(Vec::new);
    let chart_node = use_node_ref();
    let chart_size = use_size(chart_node.clone());
    let echarts = use_state(|| None::<Echarts>);

    {
        let benchmark_name = props.benchmark_name.clone();
        let hardware = hardware_ctx.state.selected_hardware.clone();
        let chart_data = chart_data.clone();

        use_effect_with(
            (benchmark_name, hardware),
            move |(benchmark_name, hardware)| {
                log!("Fetching trend data for benchmark:", benchmark_name);
                let benchmark_name = benchmark_name.clone();
                let hardware = hardware.clone();
                spawn_local(async move {
                    if let Some(hardware) = hardware {
                        match fetch_trend_data(&benchmark_name, &hardware).await {
                            Ok(data) => {
                                chart_data.set(data);
                            }
                            Err(e) => {
                                log!(format!("Error fetching trend data: {}", e));
                            }
                        }
                    }
                });
                Box::new(|| ()) as CleanupFn
            },
        );
    }

    {
        let data = (*chart_data).clone();
        let measurement_type = props.measurement_type.clone();
        let is_dark = props.is_dark;
        let echarts = echarts.clone();

        use_effect_with(
            (data, measurement_type, is_dark, chart_size),
            move |(data, measurement_type, is_dark, size)| {
                if data.is_empty() {
                    log!(format!("No data to render chart"));
                    return Box::new(|| ()) as CleanupFn;
                }

                let versions: Vec<String> = data.iter().map(|d| d.version.clone()).collect();

                let chart = match measurement_type {
                    MeasurementType::Latency => {
                        let latencies: Vec<f64> = data.iter().map(|d| d.data.latency_avg).collect();
                        let p95_latencies: Vec<f64> =
                            data.iter().map(|d| d.data.latency_p95).collect();
                        let p99_latencies: Vec<f64> =
                            data.iter().map(|d| d.data.latency_p99).collect();
                        let p999_latencies: Vec<f64> =
                            data.iter().map(|d| d.data.latency_p999).collect();

                        Chart::new()
                            .background_color(if *is_dark { "#242424" } else { "#ffffff" })
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
                                    .top(50)
                                    .data(vec![
                                        "Average Latency",
                                        "P95 Latency",
                                        "P99 Latency",
                                        "P999 Latency",
                                    ])
                                    .selected_mode(LegendSelectedMode::Multiple),
                            )
                            .grid(Grid::new().left("5%").right("5%").top("15%").bottom("10%"))
                            .x_axis(
                                Axis::new()
                                    .type_(AxisType::Category)
                                    .data(versions)
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
                    MeasurementType::Throughput => {
                        let throughput: Vec<f64> =
                            data.iter().map(|d| d.data.throughput_msgs).collect();

                        Chart::new()
                            .background_color(if *is_dark { "#242424" } else { "#ffffff" })
                            .title(
                                Title::new()
                                    .text("Throughput (Messages/s)")
                                    .left("center")
                                    .top(10)
                                    .text_style(TextStyle::new().font_size(20).font_weight("bold")),
                            )
                            .tooltip(Tooltip::new().trigger(Trigger::Axis))
                            .grid(Grid::new().left("5%").right("5%").top("15%").bottom("10%"))
                            .x_axis(
                                Axis::new()
                                    .type_(AxisType::Category)
                                    .data(versions)
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
                    MeasurementType::ThroughputMb => {
                        let throughput: Vec<f64> =
                            data.iter().map(|d| d.data.throughput_mb).collect();

                        Chart::new()
                            .background_color(if *is_dark { "#242424" } else { "#ffffff" })
                            .title(
                                Title::new()
                                    .text("Throughput (MB/s)")
                                    .left("center")
                                    .top(10)
                                    .text_style(TextStyle::new().font_size(20).font_weight("bold")),
                            )
                            .tooltip(Tooltip::new().trigger(Trigger::Axis))
                            .grid(Grid::new().left("5%").right("5%").top("15%").bottom("10%"))
                            .x_axis(
                                Axis::new()
                                    .type_(AxisType::Category)
                                    .data(versions)
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
                };

                // Get the parent container size
                let (width, height) = *size;

                // Create renderer with appropriate theme
                let renderer = if *is_dark {
                    WasmRenderer::new(width, height).theme(Theme::Dark)
                } else {
                    WasmRenderer::new(width, height).theme(Theme::Default)
                };

                // Dispose existing chart if any
                if echarts.is_some() {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(element) = document.get_element_by_id("trend-chart") {
                                if let Some(instance) = getInstanceByDom(&element) {
                                    instance.dispose();
                                }
                            }
                        }
                    }
                }

                // Render new chart
                match renderer.render("trend-chart", &chart) {
                    Ok(new_e) => echarts.set(Some(new_e)),
                    Err(e) => log!(format!("Error rendering chart: {}", e.to_string())),
                }

                Box::new(|| ()) as CleanupFn
            },
        );
    }

    let style = "width: 80%; height: 80%;";

    html! {
        <div class="trend-view" style="width: 100%; height: 100%; display: flex; justify-content: center; align-items: center;">
            <div id="trend-chart" ref={chart_node} {style}></div>
        </div>
    }
}
