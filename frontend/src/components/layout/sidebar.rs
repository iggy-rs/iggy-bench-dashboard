use crate::components::{
    benchmark_selector::BenchmarkSelector, gitref_selector::Gitref,
    hardware_selector::HardwareSelector, measurements::Measurements,
    view_mode_toggle::ViewModeToggle,
};
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
            <BenchmarkSelector />

            <div class="file-list">
                <Measurements
                    selected_file={props.selected_file.clone()}
                    on_file_select={props.on_file_select.clone()}
                />
            </div>
        </div>
    }
}
