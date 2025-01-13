use crate::components::{
    benchmark_result::BenchmarkResult, gitref_selector::Gitref,
    hardware_selector::HardwareSelector, measurements::Measurements,
    view_mode_toggle::ViewModeToggle,
};
use crate::state::benchmark::{use_benchmark, BenchmarkAction};
use crate::state::gitref::use_version;
use crate::state::view_mode::{use_view_mode, ViewMode};
use crate::types::MeasurementType;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub selected_file: MeasurementType,
    pub on_file_select: Callback<MeasurementType>,
    pub on_version_select: Callback<String>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let benchmark_ctx = use_benchmark();
    let version_ctx = use_version();
    let view_mode_ctx = use_view_mode();
    let is_trend_view = matches!(view_mode_ctx.mode, ViewMode::VersionTrend);

    html! {
        <div class="sidebar">
            <div class="logo">
                <img src="/assets/iggy.png" alt="Iggy Logo" />
                <h1>{"Iggy Benchmarks"}</h1>
            </div>

            <HardwareSelector />
            <ViewModeToggle />

            if !is_trend_view {
                <Gitref
                    versions={version_ctx.state.versions.clone()}
                    selected_version={version_ctx.state.selected_version.clone().unwrap_or_default()}
                    on_version_select={props.on_version_select.clone()}
                />
            }

            <h3>{"Benchmarks"}</h3>
            <div class="benchmark-categories">
                <div class="benchmark-category">
                    <h4>{"Send"}</h4>
                    <div class="benchmark-list">
                        {benchmark_ctx.state.benchmarks.iter()
                            .filter(|benchmark| benchmark.name.starts_with("send"))
                            .map(|benchmark| {
                                let selected = benchmark_ctx.state.selected_benchmark
                                    .as_ref()
                                    .map(|selected| selected == &benchmark.name)
                                    .unwrap_or(false);
                                html! {
                                    <BenchmarkResult
                                        kind={benchmark.name.to_string()}
                                        pretty_name={benchmark.pretty_name.clone()}
                                        selected={selected}
                                        on_select={
                                            let dispatch = benchmark_ctx.dispatch.clone();
                                            Callback::from(move |name| {
                                                dispatch.emit(BenchmarkAction::SelectBenchmark(name));
                                            })
                                        }
                                    />
                                }
                            }).collect::<Html>()}
                    </div>
                </div>

                <div class="benchmark-category">
                    <h4>{"Poll"}</h4>
                    <div class="benchmark-list">
                        {benchmark_ctx.state.benchmarks.iter()
                            .filter(|benchmark| benchmark.name.starts_with("poll"))
                            .map(|benchmark| {
                                let selected = benchmark_ctx.state.selected_benchmark
                                    .as_ref()
                                    .map(|selected| selected == &benchmark.name)
                                    .unwrap_or(false);
                                html! {
                                    <BenchmarkResult
                                        kind={benchmark.name.to_string()}
                                        pretty_name={benchmark.pretty_name.clone()}
                                        selected={selected}
                                        on_select={
                                            let dispatch = benchmark_ctx.dispatch.clone();
                                            Callback::from(move |name| {
                                                dispatch.emit(BenchmarkAction::SelectBenchmark(name));
                                            })
                                        }
                                    />
                                }
                            }).collect::<Html>()}
                    </div>
                </div>
            </div>

            <div class="file-list">
                {if benchmark_ctx.state.selected_benchmark.is_some() {
                    html! {
                        <Measurements
                            selected_file={props.selected_file.clone()}
                            on_file_select={props.on_file_select.clone()}
                        />
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}
