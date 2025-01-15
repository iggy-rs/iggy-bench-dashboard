use crate::state::benchmark::{use_benchmark, BenchmarkAction};
use shared::BenchmarkInfoFromDirectoryName;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BenchmarkType {
    Send,
    Poll,
}

impl BenchmarkType {
    fn as_str(&self) -> &'static str {
        match self {
            BenchmarkType::Send => "send",
            BenchmarkType::Poll => "poll",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BenchmarkSelectorProps {}

#[function_component(BenchmarkSelector)]
pub fn benchmark_selector(_props: &BenchmarkSelectorProps) -> Html {
    let benchmark_ctx = use_benchmark();
    let selected_type = use_state(|| BenchmarkType::Send);

    let send_benchmarks: Vec<&BenchmarkInfoFromDirectoryName> = benchmark_ctx
        .state
        .benchmarks
        .iter()
        .filter(|b| b.name.starts_with("send"))
        .collect();

    let poll_benchmarks: Vec<&BenchmarkInfoFromDirectoryName> = benchmark_ctx
        .state
        .benchmarks
        .iter()
        .filter(|b| b.name.starts_with("poll"))
        .collect();

    let on_type_select = {
        let selected_type = selected_type.clone();
        let dispatch = benchmark_ctx.dispatch.clone();
        let benchmark_ctx = benchmark_ctx.clone();
        Callback::from(move |benchmark_type: BenchmarkType| {
            // Get current benchmark name if any
            if let Some(current_name) = &benchmark_ctx.state.selected_benchmark {
                // Extract test parameters (everything after "send_" or "poll_")
                let params = current_name
                    .strip_prefix("send_")
                    .or_else(|| current_name.strip_prefix("poll_"));

                // If we have parameters, find the corresponding test in the new type
                if let Some(params) = params {
                    let new_name = format!("{}_{}", benchmark_type.as_str(), params);
                    // Only select if the benchmark exists
                    if benchmark_ctx
                        .state
                        .benchmarks
                        .iter()
                        .any(|b| b.name == new_name)
                    {
                        dispatch.emit(BenchmarkAction::SelectBenchmark(new_name));
                    }
                }
            }
            selected_type.set(benchmark_type);
        })
    };

    let on_benchmark_select = {
        let dispatch = benchmark_ctx.dispatch.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = target {
                let value = select.value();
                dispatch.emit(BenchmarkAction::SelectBenchmark(value));
            }
        })
    };

    let current_benchmarks = match *selected_type {
        BenchmarkType::Send => &send_benchmarks,
        BenchmarkType::Poll => &poll_benchmarks,
    };

    html! {
        <div class="benchmark-selector">
            <div class="view-mode-container">
                <div class="segmented-control">
                    <button
                        class={if matches!(*selected_type, BenchmarkType::Send) { "segment active" } else { "segment" }}
                        onclick={let on_type_select = on_type_select.clone();
                                Callback::from(move |_| on_type_select.emit(BenchmarkType::Send))}
                    >
                        {"Send"}
                    </button>
                    <button
                        class={if matches!(*selected_type, BenchmarkType::Poll) { "segment active" } else { "segment" }}
                        onclick={let on_type_select = on_type_select.clone();
                                Callback::from(move |_| on_type_select.emit(BenchmarkType::Poll))}
                    >
                        {"Poll"}
                    </button>
                </div>
            </div>

            <select
                onchange={on_benchmark_select}
                value={benchmark_ctx.state.selected_benchmark.clone().unwrap_or_default()}
            >
                <option value="" disabled=true selected=true>{"Select a benchmark"}</option>
                {current_benchmarks.iter().map(|benchmark| {
                    html! {
                        <option value={benchmark.name.clone()}>
                            {benchmark.pretty_name.clone().unwrap_or_else(|| benchmark.name.clone())}
                        </option>
                    }
                }).collect::<Html>()}
            </select>
        </div>
    }
}
