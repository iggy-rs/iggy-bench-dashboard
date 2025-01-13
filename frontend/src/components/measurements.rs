use crate::types::MeasurementType;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MeasurementsProps {
    pub selected_file: MeasurementType,
    pub on_file_select: Callback<MeasurementType>,
}

#[function_component(Measurements)]
pub fn measurements(props: &MeasurementsProps) -> Html {
    let on_file_select = props.on_file_select.clone();
    let selected_file = props.selected_file.clone();

    html! {
        <div class="file-list">
            <h3>{"Measurements"}</h3>
            <button
                class={classes!(
                    "file",
                    (selected_file == MeasurementType::Latency).then_some("selected")
                )}
                onclick={let on_file_select = on_file_select.clone();
                    move |_| on_file_select.emit(MeasurementType::Latency)}
            >
                { "Latency" }
            </button>
            <button
                class={classes!(
                    "file",
                    (selected_file == MeasurementType::Throughput).then_some("selected")
                )}
                onclick={let on_file_select = on_file_select.clone();
                    move |_| on_file_select.emit(MeasurementType::Throughput)}
            >
                { "Throughput" }
            </button>
            <button
                class={classes!(
                    "file",
                    (selected_file == MeasurementType::ThroughputMb).then_some("selected")
                )}
                onclick={move |_| on_file_select.emit(MeasurementType::ThroughputMb)}
            >
                { "Throughput MB" }
            </button>
        </div>
    }
}
