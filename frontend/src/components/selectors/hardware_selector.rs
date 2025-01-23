use crate::state::hardware::{use_hardware, HardwareAction};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HardwareSelectorProps {}

#[function_component(HardwareSelector)]
pub fn hardware_selector(_props: &HardwareSelectorProps) -> Html {
    let hardware_ctx = use_hardware();

    let onchange = {
        let dispatch = hardware_ctx.dispatch.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                dispatch.emit(HardwareAction::SelectHardware(target.value().parse().ok()));
            }
        })
    };

    html! {
        <div class="hardware-select">
            <h3>{"Hardware"}</h3>
            <select onchange={onchange}>
                {hardware_ctx.state.hardware_list.iter().map(|hardware| {
                    html! {
                        <option
                            value={hardware.identifier.clone().unwrap_or_else(|| "Unknown".to_string())}
                            selected={hardware_ctx.state.selected_hardware == Some(hardware.identifier.clone().unwrap_or_else(|| "Unknown".to_string()))}
                        >
                            {format!("{} @ {}", hardware.identifier.clone().unwrap_or_else(|| "Unknown".to_string()), &hardware.cpu_name)}
                        </option>
                    }
                }).collect::<Html>()}
            </select>
        </div>
    }
}
