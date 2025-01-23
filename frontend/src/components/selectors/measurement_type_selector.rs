use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MeasurementType {
    Latency,
    Throughput,
}

#[derive(Properties, PartialEq)]
pub struct MeasurementTypeSelectorProps {
    pub selected_measurement: MeasurementType,
    pub on_measurement_select: Callback<MeasurementType>,
}

#[function_component(MeasurementTypeSelector)]
pub fn measurement_type_selector(props: &MeasurementTypeSelectorProps) -> Html {
    let is_latency = matches!(props.selected_measurement, MeasurementType::Latency);

    html! {
        <div class="view-mode-container">
            <h3>{"Measurements"}</h3>
            <div class="segmented-control">
                <button
                    class={if is_latency { "segment active" } else { "segment" }}
                    onclick={props.on_measurement_select.reform(|_| MeasurementType::Latency)}
                >
                    {"Latency"}
                </button>
                <button
                    class={if !is_latency { "segment active" } else { "segment" }}
                    onclick={props.on_measurement_select.reform(|_| MeasurementType::Throughput)}
                >
                    {"Throughput"}
                </button>
            </div>
        </div>
    }
}
