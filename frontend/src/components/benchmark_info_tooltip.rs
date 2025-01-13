use shared::BenchmarkDetails;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BenchmarkInfoTooltipProps {
    pub benchmark_info: Option<BenchmarkDetails>,
    pub visible: bool,
}

#[function_component(BenchmarkInfoTooltip)]
pub fn benchmark_info_tooltip(props: &BenchmarkInfoTooltipProps) -> Html {
    if !props.visible || props.benchmark_info.is_none() {
        return html! {};
    }

    let benchmark_info = props.benchmark_info.as_ref().unwrap();
    let hardware = &benchmark_info.hardware;
    let params = &benchmark_info.params;

    html! {
        <div class="benchmark-info-tooltip">
            <div class="tooltip-section">
                <h4>{"Hardware"}</h4>
                <div class="tooltip-content">
                    <p><strong>{"CPU: "}</strong>{&hardware.cpu_name}</p>
                    <p><strong>{"Cores: "}</strong>{hardware.cpu_cores}</p>
                    <p><strong>{"Frequency: "}</strong>{hardware.cpu_frequency_mhz}{" MHz"}</p>
                    <p><strong>{"Memory: "}</strong>{hardware.total_memory_kb / 1024 / 1024}{" GB"}</p>
                    <p><strong>{"OS: "}</strong>{format!("{} {}", hardware.os_name, hardware.os_version)}</p>
                </div>
            </div>
            <div class="tooltip-section">
                <h4>{"Benchmark Parameters"}</h4>
                <div class="tooltip-content">
                    <p><strong>{"Time: "}</strong>{&params.timestamp}</p>
                    <p><strong>{"Kind: "}</strong>{&params.benchmark_kind}</p>
                    <p><strong>{"Transport: "}</strong>{&params.transport}</p>
                    <p><strong>{"Messages: "}</strong>{format!("{} x {} ({} bytes)",
                        params.message_batches,
                        params.messages_per_batch,
                        params.message_size
                    )}</p>
                    <p><strong>{"Actors: "}</strong>{format!("{} producers, {} consumers",
                        params.producers,
                        params.consumers
                    )}</p>
                    <p><strong>{"Config: "}</strong>{format!("{} streams, {} topics, {} partitions per topic",
                        params.streams,
                        params.streams,
                        params.partitions
                    )}</p>
                    <p><strong>{"Git ref: "}</strong>{&params.git_ref}</p>
                </div>
            </div>
        </div>
    }
}
