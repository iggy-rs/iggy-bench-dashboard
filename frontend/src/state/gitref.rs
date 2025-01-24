use gloo::console::log;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct GitrefState {
    pub gitrefs: Vec<String>,
    pub selected_gitref: Option<String>,
}

pub enum GitrefAction {
    SetGitrefs(Vec<String>),
    SetSelectedGitref(Option<String>),
}

#[derive(Clone, PartialEq)]
pub struct GitrefContext {
    pub state: GitrefState,
    pub dispatch: Callback<GitrefAction>,
}

impl GitrefContext {
    pub fn new(state: GitrefState, dispatch: Callback<GitrefAction>) -> Self {
        Self { state, dispatch }
    }
}

impl Reducible for GitrefState {
    type Action = GitrefAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            GitrefAction::SetGitrefs(gitrefs) => {
                log!("Available gitrefs updated:", format!("{:?}", &gitrefs));
                GitrefState {
                    gitrefs,
                    selected_gitref: self.selected_gitref.clone(),
                }
            }
            GitrefAction::SetSelectedGitref(gitref) => {
                log!("Gitref state updated to:", format!("{:?}", &gitref));
                GitrefState {
                    gitrefs: self.gitrefs.clone(),
                    selected_gitref: gitref,
                }
            }
        };

        next_state.into()
    }
}

#[derive(Properties, PartialEq)]
pub struct GitrefProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(GitrefProvider)]
pub fn gitref_provider(props: &GitrefProviderProps) -> Html {
    let state = use_reducer(GitrefState::default);

    let context = GitrefContext::new(
        (*state).clone(),
        Callback::from(move |action| state.dispatch(action)),
    );

    html! {
        <ContextProvider<GitrefContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<GitrefContext>>
    }
}

#[hook]
pub fn use_gitref() -> GitrefContext {
    use_context::<GitrefContext>().expect("Gitref context not found")
}
