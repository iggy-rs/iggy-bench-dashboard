use crate::api::fetch_benchmark_report_full;
use crate::components::chart::{dispose_chart, PlotConfig};
use crate::components::selectors::measurement_type_selector::MeasurementType;
use charming::theme::Theme;
use charming::{Echarts, WasmRenderer};
use gloo::console::log;
use iggy_bench_report::report::BenchmarkReport;
use uuid::Uuid;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::use_size;

type CleanupFn = Box<dyn FnOnce()>;

#[derive(Properties, PartialEq)]
pub struct SingleChartProps {
    pub benchmark_uuid: Uuid,
    pub measurement_type: MeasurementType,
    pub is_dark: bool,
}

#[function_component(SingleChart)]
pub fn single_chart(props: &SingleChartProps) -> Html {
    let chart_data = use_state(BenchmarkReport::default);
    let chart_node = use_node_ref();
    let chart_size = use_size(chart_node.clone());
    let echarts = use_state(|| None::<Echarts>);
    let is_loading = use_state(|| false);

    {
        let benchmark_uuid = props.benchmark_uuid;
        let chart_data = chart_data.clone();
        let is_loading = is_loading.clone();

        use_effect_with(benchmark_uuid, move |benchmark_uuid| {
            let benchmark_uuid = *benchmark_uuid;
            is_loading.set(true);

            spawn_local(async move {
                match fetch_benchmark_report_full(&benchmark_uuid).await {
                    Ok(data) => {
                        chart_data.set(data);
                        is_loading.set(false);
                    }
                    Err(e) => {
                        log!(format!("Error fetching single data: {}", e));
                        is_loading.set(false);
                    }
                }
            });
            Box::new(|| ()) as CleanupFn
        });
    }

    {
        let data = (*chart_data).clone();
        let measurement_type = props.measurement_type.clone();
        let is_dark = props.is_dark;
        let echarts = echarts.clone();
        let is_loading = is_loading.clone();

        use_effect_with(
            (data, measurement_type, is_dark, chart_size, *is_loading),
            move |(data, measurement_type, is_dark, size, is_loading)| {
                let (width, height) = *size;
                let config = PlotConfig {
                    width,
                    height,
                    is_dark: *is_dark,
                    element_id: "single-chart-canvas".to_string(),
                };

                // Dispose existing chart if any
                if echarts.is_some() {
                    dispose_chart("single-chart-canvas");
                }

                // Only render if we're not loading
                if !is_loading {
                    let chart = match measurement_type {
                        MeasurementType::Latency => {
                            iggy_bench_report::create_latency_chart(data, config.is_dark)
                        }
                        MeasurementType::Throughput => {
                            iggy_bench_report::create_throughput_chart(data, config.is_dark)
                        }
                    };

                    let renderer = if config.is_dark {
                        WasmRenderer::new(config.width, config.height).theme(Theme::Dark)
                    } else {
                        WasmRenderer::new(config.width, config.height).theme(Theme::Default)
                    };

                    match renderer.render(&config.element_id, &chart) {
                        Ok(chart) => {
                            echarts.set(Some(chart));
                        }
                        Err(e) => {
                            log!(format!("Error rendering chart: {}", e));
                        }
                    }
                }

                Box::new(|| ()) as CleanupFn
            },
        );
    }

    let loading_class = if *is_loading { "visible" } else { "" };
    let chart_class = if *is_loading { "loading" } else { "" };

    html! {
        <div ref={chart_node} id="single-chart" style="width: 100%; height: 100%;">
            <div class={classes!("chart-container", chart_class)}>
                <div id="single-chart-canvas" style="width: 100%; height: 100%;"></div>
            </div>
            <div class={classes!("loading-overlay", loading_class)}>
                <div class="loading-spinner"></div>
            </div>
        </div>
    }
}
