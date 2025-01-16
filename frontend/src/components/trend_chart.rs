use super::plot::trend::create_chart;
use crate::components::plot::{dispose_chart, PlotConfig, PlotType};
use crate::types::MeasurementType;
use crate::{components::plot::TrendPlotData, state::hardware::use_hardware};
use charming::Echarts;
use gloo::console::log;
use gloo::net::http::Request;
use gloo::net::Error;
use shared::BenchmarkTrendData;
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
                let plot_data = TrendPlotData {
                    versions,
                    data: data.clone(),
                };

                let plot_type = match measurement_type {
                    MeasurementType::Latency => PlotType::Latency,
                    MeasurementType::Throughput => PlotType::Throughput,
                    MeasurementType::ThroughputMb => PlotType::ThroughputMb,
                };

                let (width, height) = *size;
                let config = PlotConfig {
                    width,
                    height,
                    is_dark: *is_dark,
                    element_id: "trend-chart".to_string(),
                };

                // Dispose existing chart if any
                if echarts.is_some() {
                    dispose_chart("trend-chart");
                }

                // Render new chart
                match create_chart(&config, &plot_data, &plot_type) {
                    Ok(new_e) => echarts.set(Some(new_e)),
                    Err(e) => log!(format!("Error rendering chart: {}", e)),
                }

                Box::new(|| ()) as CleanupFn
            },
        );
    }

    html! {
        <div ref={chart_node} id="trend-chart" style="width: calc(100% - 40px); height: calc(100% - 20px); margin: 10px 20px;"></div>
    }
}
