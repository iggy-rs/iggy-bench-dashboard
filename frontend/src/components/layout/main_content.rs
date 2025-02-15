use crate::components::chart::single_chart::SingleChart;
use crate::components::chart::trend_chart::TrendChart;
use crate::components::layout::topbar::TopBar;
use crate::state::benchmark::use_benchmark;
use crate::state::ui::{use_ui, ViewMode};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainContentProps {
    pub selected_gitref: String,
    pub is_dark: bool,
    pub on_theme_toggle: Callback<bool>,
    pub view_mode: ViewMode,
}

#[function_component(MainContent)]
pub fn main_content(props: &MainContentProps) -> Html {
    let benchmark_ctx = use_benchmark();
    let ui = use_ui();
    let selected_measurement = ui.selected_measurement.clone();

    let content = if let Some(selected_benchmark) = &benchmark_ctx.state.selected_benchmark {
        match props.view_mode {
            ViewMode::SingleGitref => {
                html! {
                    <div class="content-wrapper">
                            <div class="chart-title">
                            <div class="chart-title-primary">
                                { selected_benchmark.title(&selected_measurement.to_string()) }
                            </div>
                            <div class="chart-title-sub">
                                { selected_benchmark.subtext() }
                            </div>
                        </div>
                        <div class="single-view">
                            <SingleChart
                                benchmark_uuid={selected_benchmark.uuid}
                                measurement_type={selected_measurement.clone()}
                                is_dark={props.is_dark}
                            />
                        </div>
                    </div>
                }
            }
            ViewMode::GitrefTrend => {
                html! {
                    <div class="content-wrapper">
                        <div class="chart-title">
                            <div class="chart-title-primary">
                                { selected_benchmark.title(&selected_measurement.to_string()) }
                            </div>
                            <div class="chart-title-sub">
                                { selected_benchmark.params.format_params() }
                            </div>
                        </div>
                        <div class="trend-view">
                            <TrendChart
                                params_identifier={selected_benchmark.params.params_identifier.clone()}
                                measurement_type={selected_measurement.clone()}
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
        <div class="content">
            <TopBar
                is_dark={props.is_dark}
                selected_gitref={props.selected_gitref.clone()}
                on_theme_toggle={props.on_theme_toggle.clone()}
            />
            {content}
        </div>
    }
}
