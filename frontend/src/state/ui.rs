use crate::components::selectors::measurement_type_selector::MeasurementType;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ViewMode {
    SingleGitref, // View detailed performance for a specific gitref
    GitrefTrend,  // View performance trends across all gitrefs
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiState {
    pub view_mode: ViewMode,
    pub selected_measurement: MeasurementType,
    pub is_benchmark_tooltip_visible: bool,
    pub is_server_stats_tooltip_visible: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            view_mode: ViewMode::SingleGitref,
            selected_measurement: MeasurementType::Latency,
            is_benchmark_tooltip_visible: false,
            is_server_stats_tooltip_visible: false,
        }
    }
}

pub enum UiAction {
    SetMeasurementType(MeasurementType),
    ToggleBenchmarkTooltip,
    ToggleServerStatsTooltip,
    SetViewMode(ViewMode),
}

impl Reducible for UiState {
    type Action = UiAction;

    fn reduce(self: Rc<Self>, action: UiAction) -> Rc<Self> {
        let next = match action {
            UiAction::SetMeasurementType(mt) => UiState {
                selected_measurement: mt,
                ..(*self).clone()
            },
            UiAction::ToggleBenchmarkTooltip => UiState {
                is_benchmark_tooltip_visible: !self.is_benchmark_tooltip_visible,
                ..(*self).clone()
            },
            UiAction::ToggleServerStatsTooltip => UiState {
                is_server_stats_tooltip_visible: !self.is_server_stats_tooltip_visible,
                ..(*self).clone()
            },
            UiAction::SetViewMode(vm) => UiState {
                view_mode: vm,
                ..(*self).clone()
            },
        };
        next.into()
    }
}

#[derive(Properties, PartialEq)]
pub struct UiProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(UiProvider)]
pub fn ui_provider(props: &UiProviderProps) -> Html {
    let state = use_reducer(UiState::default);

    html! {
        <ContextProvider<UseReducerHandle<UiState>> context={state}>
            { for props.children.iter() }
        </ContextProvider<UseReducerHandle<UiState>>>
    }
}

#[hook]
pub fn use_ui() -> UseReducerHandle<UiState> {
    use_context::<UseReducerHandle<UiState>>().expect("Ui context not found")
}
