use super::logo::Logo;
use crate::components::selectors::benchmark_selector::BenchmarkSelector;
use crate::components::selectors::gitref_selector::GitrefSelector;
use crate::components::selectors::hardware_selector::HardwareSelector;
use crate::components::selectors::measurement_type_selector::{
    MeasurementType, MeasurementTypeSelector,
};
use crate::components::view_mode_toggle::ViewModeToggle;
use crate::state::gitref::use_gitref;
use crate::state::view_mode::{use_view_mode, ViewMode};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub selected_measurement: MeasurementType,
    pub on_measurement_select: Callback<MeasurementType>,
    pub on_gitref_select: Callback<String>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let gitref_ctx = use_gitref();
    let view_mode_ctx = use_view_mode();
    let is_trend_view = matches!(view_mode_ctx.mode, ViewMode::GitrefTrend);

    html! {
        <div class="sidebar">
            <Logo />
            <HardwareSelector />
            <ViewModeToggle />

            if !is_trend_view {
                <GitrefSelector
                    gitrefs={gitref_ctx.state.gitrefs.clone()}
                    selected_gitref={gitref_ctx.state.selected_gitref.clone().unwrap_or_default()}
                    on_gitref_select={props.on_gitref_select.clone()}
                />
            }

            <BenchmarkSelector />

            <div class="measurement-type-selector">
                <MeasurementTypeSelector
                    selected_measurement={props.selected_measurement.clone()}
                    on_measurement_select={props.on_measurement_select.clone()}
                />
            </div>
        </div>
    }
}
