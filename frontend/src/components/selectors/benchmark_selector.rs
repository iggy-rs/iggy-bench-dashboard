use crate::{
    components::selectors::benchmark_kind_selector::BenchmarkKindSelector,
    state::benchmark::{use_benchmark, BenchmarkAction},
};
use iggy_benchmark_report::benchmark_kind::BenchmarkKind;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BenchmarkSelectorProps {}

#[function_component(BenchmarkSelector)]
pub fn benchmark_selector(_props: &BenchmarkSelectorProps) -> Html {
    let benchmark_ctx = use_benchmark();
    let selected_kind = benchmark_ctx.state.selected_kind;

    // Create a longer-lived reference to the current benchmarks
    let empty_vec = Vec::new();
    let current_benchmarks = benchmark_ctx
        .state
        .entries
        .get(&selected_kind)
        .unwrap_or(&empty_vec);

    let on_benchmark_select = {
        let dispatch = benchmark_ctx.dispatch.clone();
        let entries = benchmark_ctx.state.entries.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = target {
                let value = select.value();
                let selected_benchmark = entries.get(&selected_kind).and_then(|benchmarks| {
                    benchmarks.iter().find(|b| b.params.pretty_name == value)
                });
                dispatch.emit(BenchmarkAction::SelectBenchmark(Box::new(
                    selected_benchmark.cloned(),
                )));
            }
        })
    };

    let on_kind_select = {
        let dispatch = benchmark_ctx.dispatch.clone();
        Callback::from(move |kind: BenchmarkKind| {
            dispatch.emit(BenchmarkAction::SelectBenchmarkKind(kind));
        })
    };

    // Get the current benchmark's pretty_name if it exists
    let current_value = benchmark_ctx
        .state
        .selected_benchmark
        .as_ref()
        .map(|b| b.params.pretty_name.clone())
        .unwrap_or_default();

    html! {
        <div class="benchmark-select">
            <BenchmarkKindSelector
                selected_kind={selected_kind}
                on_kind_select={on_kind_select}
            />

            <select
                onchange={on_benchmark_select}
                value={current_value.clone()}
            >
                {current_benchmarks.iter().map(|benchmark| {
                    html! {
                        <option
                            value={benchmark.params.pretty_name.clone()}
                            selected={benchmark.params.pretty_name == current_value}
                        >
                            {benchmark.params.pretty_name.clone()}
                        </option>
                    }
                }).collect::<Html>()}
            </select>
        </div>
    }
}
