use crate::state::hardware::{use_hardware, HardwareAction};
use serde::{Deserialize, Serialize};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub hostname: String,
    pub cpu_name: String,
    pub cpu_cores: u32,
    pub cpu_frequency_mhz: u32,
    pub total_memory_kb: u64,
    pub os_name: String,
    pub os_version: String,
}

#[derive(Properties, PartialEq)]
pub struct HardwareSelectorProps {}

#[function_component(HardwareSelector)]
pub fn hardware_selector(_props: &HardwareSelectorProps) -> Html {
    let hardware_ctx = use_hardware();

    let onchange = {
        let dispatch = hardware_ctx.dispatch.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                dispatch.emit(HardwareAction::SelectHardware(target.value()));
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
                            value={hardware.hostname.clone()}
                            selected={hardware_ctx.state.selected_hardware.as_ref().map_or(false, |selected| selected == &hardware.hostname)}
                        >
                            {format!("{} @ {}", &hardware.hostname, &hardware.cpu_name)}
                        </option>
                    }
                }).collect::<Html>()}
            </select>
        </div>
    }
}
