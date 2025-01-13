use crate::components::trend_chart::TrendChart;
use crate::state::benchmark::use_benchmark;
use crate::state::hardware::use_hardware;
use crate::state::view_mode::{use_view_mode, ViewMode};
use crate::types::MeasurementType;
use crate::utils::get_file_prefix;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainContentProps {
    pub selected_file: MeasurementType,
    pub selected_version: String,
    pub is_dark: bool,
}

#[function_component(MainContent)]
pub fn main_content(props: &MainContentProps) -> Html {
    let hardware_ctx = use_hardware();
    let benchmark_ctx = use_benchmark();
    let view_mode_ctx = use_view_mode();

    let content = if let Some(selected_benchmark) = &benchmark_ctx.state.selected_benchmark {
        match view_mode_ctx.mode {
            ViewMode::SingleVersion => {
                if !props.selected_version.is_empty() {
                    let hardware = hardware_ctx
                        .state
                        .selected_hardware
                        .clone()
                        .unwrap_or_default();
                    let benchmark_path = format!(
                        "{}_{}_{}",
                        selected_benchmark, props.selected_version, hardware
                    );
                    html! {
                        <div class="content-wrapper">
                            <iframe
                                src={format!("/performance_results/{}/{}_{}",
                                    benchmark_path,
                                    get_file_prefix(selected_benchmark),
                                    props.selected_file.to_filename())}
                                class="content-frame"
                            />
                        </div>
                    }
                } else {
                    html! {
                        <div class="content-wrapper">
                            <div class="empty-state">
                                <div class="empty-state-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"/>
                                        <polyline points="14 2 14 8 20 8"/>
                                        <path d="M3 15h6"/>
                                        <path d="M3 18h6"/>
                                        <path d="M3 12h6"/>
                                    </svg>
                                    <h2>{"Select a version to view results"}</h2>
                                    <p>{"Choose a version from the dropdown menu to display benchmark data."}</p>
                                </div>
                            </div>
                        </div>
                    }
                }
            }
            ViewMode::VersionTrend => {
                html! {
                    <div class="content-wrapper">
                        <div class="trend-view">
                            <TrendChart
                                benchmark_name={selected_benchmark.clone()}
                                measurement_type={props.selected_file.clone()}
                                is_dark={props.is_dark}
                            />
                        </div>
                    </div>
                }
            }
        }
    } else {
        html! {
            <div class="content-wrapper">
                <div class="empty-state">
                    <div class="empty-state-content">
                        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
                            <polyline points="13 2 13 9 20 9"/>
                            <line x1="16" y1="13" x2="8" y2="13"/>
                            <line x1="16" y1="17" x2="8" y2="17"/>
                        </svg>
                        <h2>{"Select a benchmark to view results"}</h2>
                        <p>{"Choose a benchmark from the sidebar to display performance data."}</p>
                    </div>
                </div>
            </div>
        }
    };

    html! {
        { content }
    }
}
