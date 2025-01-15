use crate::components::single_chart::SingleChart;
use crate::components::trend_chart::TrendChart;
use crate::state::benchmark::use_benchmark;
use crate::state::view_mode::{use_view_mode, ViewMode};
use crate::types::MeasurementType;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainContentProps {
    pub selected_file: MeasurementType,
    pub selected_version: String,
    pub is_dark: bool,
}

#[function_component(MainContent)]
pub fn main_content(props: &MainContentProps) -> Html {
    let benchmark_ctx = use_benchmark();
    let view_mode_ctx = use_view_mode();

    let content = if let Some(selected_benchmark) = &benchmark_ctx.state.selected_benchmark {
        match view_mode_ctx.mode {
            ViewMode::SingleVersion => {
                html! {
                    <div class="content-wrapper">
                        <div class="single-view">
                            <SingleChart
                                benchmark_name={selected_benchmark.clone()}
                                measurement_type={props.selected_file.clone()}
                                is_dark={props.is_dark}
                                version={props.selected_version.clone()}
                            />
                        </div>
                    </div>
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
