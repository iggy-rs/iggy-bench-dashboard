use crate::api;
use crate::components::selectors::measurement_type_selector::MeasurementType;
use crate::components::theme::theme_toggle::ThemeToggle;
use crate::components::tooltips::benchmark_info_toggle::BenchmarkInfoToggle;
use crate::components::tooltips::benchmark_info_tooltip::BenchmarkInfoTooltip;
use crate::components::tooltips::server_stats_toggle::ServerStatsToggle;
use crate::components::tooltips::server_stats_tooltip::ServerStatsTooltip;
use crate::state::benchmark::use_benchmark;
use crate::state::view_mode::use_view_mode;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TopBarProps {
    pub is_dark: bool,
    pub is_benchmark_tooltip_visible: bool,
    pub selected_gitref: String,
    pub selected_measurement: MeasurementType,
    pub on_theme_toggle: Callback<bool>,
    pub on_benchmark_tooltip_toggle: Callback<()>,
    pub on_measurement_select: Callback<MeasurementType>,
}

#[function_component(TopBar)]
pub fn topbar(props: &TopBarProps) -> Html {
    let benchmark_ctx = use_benchmark();
    let view_mode_ctx = use_view_mode();
    let is_server_stats_visible = use_state(|| false);

    let on_download_artifacts = {
        let benchmark_ctx = benchmark_ctx.clone();
        Callback::from(move |_| {
            if let Some(benchmark) = &benchmark_ctx.state.selected_benchmark {
                api::download_test_artifacts(&benchmark.uuid);
            }
        })
    };

    let on_server_stats_toggle = {
        let is_server_stats_visible = is_server_stats_visible.clone();
        Callback::from(move |_| {
            is_server_stats_visible.set(!*is_server_stats_visible);
        })
    };

    html! {
        <div class="top-buttons">
            <div class="controls">
                <ThemeToggle
                    is_dark={props.is_dark}
                    on_toggle={props.on_theme_toggle.clone()}
                />
                {
                    if !props.selected_gitref.is_empty() {
                        html! {
                            <>
                                <button
                                    class="download-button"
                                    onclick={on_download_artifacts.clone()}
                                    title="Download Test Artifacts"
                                    disabled={benchmark_ctx.state.selected_benchmark.is_none()}
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                                        <polyline points="7 10 12 15 17 10"/>
                                        <line x1="12" y1="15" x2="12" y2="3"/>
                                    </svg>
                                </button>
                                <div class="info-container">
                                    <ServerStatsToggle
                                        is_visible={*is_server_stats_visible}
                                        on_toggle={on_server_stats_toggle.clone()}
                                    />
                                    {
                                        if *is_server_stats_visible {
                                            html! {
                                                <ServerStatsTooltip
                                                    benchmark_report={benchmark_ctx.state.selected_benchmark.clone()}
                                                    visible={true}
                                                    view_mode={view_mode_ctx.mode.clone()}
                                                />
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                                <div class="info-container">
                                    <BenchmarkInfoToggle
                                        is_visible={props.is_benchmark_tooltip_visible}
                                        on_toggle={props.on_benchmark_tooltip_toggle.clone()}
                                    />
                                    {
                                        if props.is_benchmark_tooltip_visible && benchmark_ctx.state.selected_benchmark.is_some() {
                                            html! {
                                                <BenchmarkInfoTooltip
                                                    benchmark_report={benchmark_ctx.state.selected_benchmark.clone().unwrap()}
                                                    visible={true}
                                                    view_mode={view_mode_ctx.mode.clone()}
                                                />
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                                <div class="measurement-buttons">
                                    <button
                                        class={classes!(
                                            "measurement-button",
                                            (props.selected_measurement == MeasurementType::Latency).then_some("active")
                                        )}
                                        onclick={props.on_measurement_select.reform(|_| MeasurementType::Latency)}
                                    >
                                        { "Latency" }
                                    </button>
                                    <button
                                        class={classes!(
                                            "measurement-button",
                                            (props.selected_measurement == MeasurementType::Throughput).then_some("active")
                                        )}
                                        onclick={props.on_measurement_select.reform(|_| MeasurementType::Throughput)}
                                    >
                                        { "Throughput" }
                                    </button>
                                </div>
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
