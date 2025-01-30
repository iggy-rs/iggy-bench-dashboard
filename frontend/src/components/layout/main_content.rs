use crate::components::chart::single_chart::SingleChart;
use crate::components::chart::trend_chart::TrendChart;
use crate::components::layout::topbar::TopBar;
use crate::components::selectors::measurement_type_selector::MeasurementType;
use crate::state::benchmark::use_benchmark;
use crate::state::view_mode::ViewMode;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainContentProps {
    pub selected_measurement: MeasurementType,
    pub selected_gitref: String,
    pub is_dark: bool,
    pub is_benchmark_tooltip_visible: bool,
    pub on_theme_toggle: Callback<bool>,
    pub on_benchmark_tooltip_toggle: Callback<()>,
    pub on_measurement_select: Callback<MeasurementType>,
    pub view_mode: ViewMode,
}

#[function_component(MainContent)]
pub fn main_content(props: &MainContentProps) -> Html {
    let benchmark_ctx = use_benchmark();

    let content = if let Some(selected_benchmark) = &benchmark_ctx.state.selected_benchmark {
        match props.view_mode {
            ViewMode::SingleGitref => {
                html! {
                    <div class="content-wrapper">
                        <div class="single-view">
                            <SingleChart
                                benchmark_uuid={selected_benchmark.uuid}
                                measurement_type={props.selected_measurement.clone()}
                                is_dark={props.is_dark}
                            />
                        </div>
                    </div>
                }
            }
            ViewMode::GitrefTrend => {
                html! {
                    <div class="content-wrapper">
                        <div class="trend-view">
                            <TrendChart
                                params_identifier={selected_benchmark.params.params_identifier.clone()}
                                measurement_type={props.selected_measurement.clone()}
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
                is_benchmark_tooltip_visible={props.is_benchmark_tooltip_visible}
                selected_gitref={props.selected_gitref.clone()}
                selected_measurement={props.selected_measurement.clone()}
                on_theme_toggle={props.on_theme_toggle.clone()}
                on_benchmark_tooltip_toggle={props.on_benchmark_tooltip_toggle.clone()}
                on_measurement_select={props.on_measurement_select.clone()}
            />
            {content}
        </div>
    }
}
