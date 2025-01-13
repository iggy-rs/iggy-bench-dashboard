use gloo::console::log;
use shared::BenchmarkHardware;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct HardwareState {
    pub hardware_list: Vec<BenchmarkHardware>,
    pub selected_hardware: Option<String>,
}

pub enum HardwareAction {
    SetHardwareList(Vec<BenchmarkHardware>),
    SelectHardware(String),
}

#[derive(Clone, PartialEq)]
pub struct HardwareContext {
    pub state: HardwareState,
    pub dispatch: Callback<HardwareAction>,
}

impl HardwareContext {
    pub fn new(state: HardwareState, dispatch: Callback<HardwareAction>) -> Self {
        Self { state, dispatch }
    }
}

impl Reducible for HardwareState {
    type Action = HardwareAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            HardwareAction::SetHardwareList(hardware_list) => {
                log!(
                    "Available hardware updated:",
                    format!("{:?}", &hardware_list)
                );
                HardwareState {
                    hardware_list,
                    selected_hardware: self.selected_hardware.clone(),
                }
            }
            HardwareAction::SelectHardware(hardware) => {
                log!("Hardware selection updated to:", format!("{:?}", &hardware));
                HardwareState {
                    hardware_list: self.hardware_list.clone(),
                    selected_hardware: Some(hardware),
                }
            }
        };

        next_state.into()
    }
}

#[derive(Properties, PartialEq)]
pub struct HardwareProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(HardwareProvider)]
pub fn hardware_provider(props: &HardwareProviderProps) -> Html {
    let state = use_reducer(HardwareState::default);

    let context = HardwareContext::new(
        (*state).clone(),
        Callback::from(move |action| state.dispatch(action)),
    );

    html! {
        <ContextProvider<HardwareContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<HardwareContext>>
    }
}

#[hook]
pub fn use_hardware() -> HardwareContext {
    use_context::<HardwareContext>().expect("Hardware context not found")
}
