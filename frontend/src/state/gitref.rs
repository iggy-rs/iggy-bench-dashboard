use gloo::console::log;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct VersionState {
    pub versions: Vec<String>,
    pub selected_version: Option<String>,
}

pub enum VersionAction {
    SetVersions(Vec<String>),
    SetSelectedVersion(Option<String>),
}

#[derive(Clone, PartialEq)]
pub struct VersionContext {
    pub state: VersionState,
    pub dispatch: Callback<VersionAction>,
}

impl VersionContext {
    pub fn new(state: VersionState, dispatch: Callback<VersionAction>) -> Self {
        Self { state, dispatch }
    }
}

impl Reducible for VersionState {
    type Action = VersionAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            VersionAction::SetVersions(versions) => {
                log!("Available versions updated:", format!("{:?}", &versions));
                VersionState {
                    versions,
                    selected_version: self.selected_version.clone(),
                }
            }
            VersionAction::SetSelectedVersion(version) => {
                log!("Version state updated to:", format!("{:?}", &version));
                VersionState {
                    versions: self.versions.clone(),
                    selected_version: version,
                }
            }
        };

        next_state.into()
    }
}

#[derive(Properties, PartialEq)]
pub struct VersionProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(VersionProvider)]
pub fn version_provider(props: &VersionProviderProps) -> Html {
    let state = use_reducer(VersionState::default);

    let context = VersionContext::new(
        (*state).clone(),
        Callback::from(move |action| state.dispatch(action)),
    );

    html! {
        <ContextProvider<VersionContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<VersionContext>>
    }
}

#[hook]
pub fn use_version() -> VersionContext {
    use_context::<VersionContext>().expect("Version context not found")
}
