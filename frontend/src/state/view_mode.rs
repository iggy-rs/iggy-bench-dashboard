use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ViewMode {
    SingleGitref, // View detailed performance for a specific gitref
    GitrefTrend,  // View performance trends across all gitrefs
}

impl Default for ViewMode {
    fn default() -> Self {
        Self::SingleGitref
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ViewModeState {
    pub mode: ViewMode,
}

pub enum ViewModeAction {
    ToggleMode,
}

impl Reducible for ViewModeState {
    type Action = ViewModeAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let next_state = match action {
            ViewModeAction::ToggleMode => Self {
                mode: match self.mode {
                    ViewMode::SingleGitref => ViewMode::GitrefTrend,
                    ViewMode::GitrefTrend => ViewMode::SingleGitref,
                },
            },
        };

        next_state.into()
    }
}

#[derive(Properties, PartialEq)]
pub struct ViewModeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ViewModeProvider)]
pub fn view_mode_provider(props: &ViewModeProviderProps) -> Html {
    let view_mode_state = use_reducer(ViewModeState::default);

    html! {
        <ContextProvider<UseReducerHandle<ViewModeState>> context={view_mode_state}>
            { for props.children.iter() }
        </ContextProvider<UseReducerHandle<ViewModeState>>>
    }
}

#[hook]
pub fn use_view_mode() -> UseReducerHandle<ViewModeState> {
    use_context::<UseReducerHandle<ViewModeState>>().unwrap()
}
