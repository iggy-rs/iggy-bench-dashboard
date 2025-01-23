use iggy_benchmark_report::benchmark_kind::BenchmarkKind;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BenchmarkKindSelectorProps {
    pub selected_kind: BenchmarkKind,
    pub on_kind_select: Callback<BenchmarkKind>,
}

#[function_component(BenchmarkKindSelector)]
pub fn benchmark_kind_selector(props: &BenchmarkKindSelectorProps) -> Html {
    html! {
        <div class="view-mode-container">
            <h3>{"Benchmark Kind"}</h3>
            <div class="segmented-control">
                <button
                    class={if matches!(props.selected_kind, BenchmarkKind::Send) { "segment active" } else { "segment" }}
                    onclick={props.on_kind_select.reform(|_| BenchmarkKind::Send)}
                >
                    {"Send"}
                </button>
                <button
                    class={if matches!(props.selected_kind, BenchmarkKind::Poll) { "segment active" } else { "segment" }}
                    onclick={props.on_kind_select.reform(|_| BenchmarkKind::Poll)}
                >
                    {"Poll"}
                </button>
                <button
                    class={if matches!(props.selected_kind, BenchmarkKind::SendAndPoll) { "segment active" } else { "segment" }}
                    onclick={props.on_kind_select.reform(|_| BenchmarkKind::SendAndPoll)}
                >
                    {"Send & Poll"}
                </button>
                <button
                    class={if matches!(props.selected_kind, BenchmarkKind::ConsumerGroupPoll) { "segment active" } else { "segment" }}
                    onclick={props.on_kind_select.reform(|_| BenchmarkKind::ConsumerGroupPoll)}
                >
                    {"Consumers Group Poll"}
                </button>
            </div>
        </div>
    }
}
