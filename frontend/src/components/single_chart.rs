use crate::components::plot::{dispose_chart, PlotConfig, PlotType};
use crate::state::hardware::use_hardware;
use crate::types::MeasurementType;
use charming::Echarts;
use gloo::console::log;
use gloo::net::http::Request;
use gloo::net::Error;
use shared::BenchmarkInfo;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::use_size;

use super::plot::single::create_chart;

type CleanupFn = Box<dyn FnOnce()>;

#[derive(Properties, PartialEq)]
pub struct SingleChartProps {
    pub benchmark_name: String,
    pub measurement_type: MeasurementType,
    pub is_dark: bool,
    pub version: String,
}

async fn fetch_single_data(
    benchmark: &str,
    hardware: &str,
    version: &str,
) -> Result<BenchmarkInfo, Error> {
    let url = format!("/api/single/{}_{}_{}", benchmark, version, hardware);
    let resp = Request::get(&url).send().await?;
    resp.json().await
}

#[function_component(SingleChart)]
pub fn single_chart(props: &SingleChartProps) -> Html {
    let hardware_ctx = use_hardware();
    let chart_data = use_state(BenchmarkInfo::default);
    let chart_node = use_node_ref();
    let chart_size = use_size(chart_node.clone());
    let echarts = use_state(|| None::<Echarts>);

    {
        let benchmark_name = props.benchmark_name.clone();
        let version = props.version.clone();
        let hardware = hardware_ctx.state.selected_hardware.clone();
        let chart_data = chart_data.clone();

        use_effect_with(
            (benchmark_name, hardware, version),
            move |(benchmark_name, hardware, version)| {
                log!(
                    "Fetching single data - benchmark: {}, hardware: {}, version: {}",
                    benchmark_name,
                    hardware.clone().unwrap_or_default(),
                    version
                );
                let benchmark_name = benchmark_name.clone();
                let version = version.clone();
                let hardware = hardware.clone();
                spawn_local(async move {
                    if let Some(hardware) = hardware {
                        match fetch_single_data(&benchmark_name, &hardware, &version).await {
                            Ok(data) => {
                                chart_data.set(data);
                            }
                            Err(e) => {
                                log!(format!("Error fetching single data: {}", e));
                            }
                        }
                    } else {
                        log!("No hardware selected");
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
                    element_id: "single-chart".to_string(),
                };

                // Dispose existing chart if any
                if echarts.is_some() {
                    dispose_chart("single-chart");
                }

                // Render new chart
                match create_chart(&config, data, &plot_type) {
                    Ok(new_e) => {
                        log!("Successfully created chart");
                        echarts.set(Some(new_e))
                    }
                    Err(e) => log!("Error rendering chart:", e),
                }

                Box::new(|| ()) as CleanupFn
            },
        );
    }

    html! {
        <div ref={chart_node} id="single-chart" style="width: 100%; height: 100%;"></div>
    }
}
