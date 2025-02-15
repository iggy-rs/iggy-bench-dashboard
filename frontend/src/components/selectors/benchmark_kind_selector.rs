use crate::state::benchmark::use_benchmark;
use iggy_bench_report::benchmark_kind::BenchmarkKind;
use std::collections::HashSet;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BenchmarkKindSelectorProps {
    pub selected_kind: BenchmarkKind,
    pub on_kind_select: Callback<BenchmarkKind>,
    pub available_kinds: HashSet<BenchmarkKind>,
}

#[function_component(BenchmarkKindSelector)]
pub fn benchmark_kind_selector(props: &BenchmarkKindSelectorProps) -> Html {
    let benchmark_ctx = use_benchmark();

    let count_benchmarks = |kind: BenchmarkKind| -> usize {
        benchmark_ctx
            .state
            .entries
            .values()
            .map(|benchmarks| {
                benchmarks
                    .iter()
                    .filter(|b| b.params.benchmark_kind == kind)
                    .count()
            })
            .sum()
    };

    html! {
        <div class="benchmark-grid">
            if matches!(props.selected_kind,
                BenchmarkKind::PinnedProducer |
                BenchmarkKind::PinnedConsumer |
                BenchmarkKind::PinnedProducerAndConsumer)
            {
                <>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::PinnedProducer).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::PinnedProducer)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::PinnedProducer)
                        }
                    >
                        <span class="benchmark-option-icon">{"↑"}</span>
                        <span class="benchmark-option-label">{"Producer ("}{count_benchmarks(BenchmarkKind::PinnedProducer)}{")"}</span>
                    </button>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::PinnedConsumer).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::PinnedConsumer)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::PinnedConsumer)
                        }
                    >
                        <span class="benchmark-option-icon">{"↓"}</span>
                        <span class="benchmark-option-label">{"Consumer ("}{count_benchmarks(BenchmarkKind::PinnedConsumer)}{")"}</span>
                    </button>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::PinnedProducerAndConsumer).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::PinnedProducerAndConsumer)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::PinnedProducerAndConsumer)
                        }
                    >
                        <span class="benchmark-option-icon">{"↕"}</span>
                        <span class="benchmark-option-label">{"Producer & Consumer ("}{count_benchmarks(BenchmarkKind::PinnedProducerAndConsumer)}{")"}</span>
                    </button>
                </>
            } else if matches!(props.selected_kind,
                BenchmarkKind::BalancedProducer |
                BenchmarkKind::BalancedConsumerGroup |
                BenchmarkKind::BalancedProducerAndConsumerGroup)
            {
                <>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::BalancedProducer).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::BalancedProducer)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::BalancedProducer)
                        }
                    >
                        <span class="benchmark-option-icon">{"↑"}</span>
                        <span class="benchmark-option-label">{"Producer ("}{count_benchmarks(BenchmarkKind::BalancedProducer)}{")"}</span>
                    </button>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::BalancedConsumerGroup).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::BalancedConsumerGroup)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::BalancedConsumerGroup)
                        }
                    >
                        <span class="benchmark-option-icon">{"↓"}</span>
                        <span class="benchmark-option-label">{"Consumer Group ("}{count_benchmarks(BenchmarkKind::BalancedConsumerGroup)}{")"}</span>
                    </button>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::BalancedProducerAndConsumerGroup).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::BalancedProducerAndConsumerGroup)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::BalancedProducerAndConsumerGroup)
                        }
                    >
                        <span class="benchmark-option-icon">{"↕"}</span>
                        <span class="benchmark-option-label">{"Producer & Consumer Group ("}{count_benchmarks(BenchmarkKind::BalancedProducerAndConsumerGroup)}{")"}</span>
                    </button>
                </>
            } else {
                <>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::EndToEndProducingConsumer).then_some("active"),
                            ((!matches!(props.selected_kind, BenchmarkKind::EndToEndProducingConsumer))
                                && (!props.available_kinds.contains(&BenchmarkKind::EndToEndProducingConsumer)))
                                .then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::EndToEndProducingConsumer)
                        }
                    >
                        <span class="benchmark-option-icon">{"↔"}</span>
                        <span class="benchmark-option-label">
                            {"Producing Consumer ("}{count_benchmarks(BenchmarkKind::EndToEndProducingConsumer)}{")"}
                        </span>
                    </button>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::EndToEndProducingConsumerGroup).then_some("active"),
                            ((!matches!(props.selected_kind, BenchmarkKind::EndToEndProducingConsumerGroup))
                                && (!props.available_kinds.contains(&BenchmarkKind::EndToEndProducingConsumerGroup)))
                                .then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::EndToEndProducingConsumerGroup)
                        }
                    >
                        <span class="benchmark-option-icon">{"↔"}</span>
                        <span class="benchmark-option-label">
                            {"Producing Consumer Group ("}{count_benchmarks(BenchmarkKind::EndToEndProducingConsumerGroup)}{")"}
                        </span>
                    </button>
                </>
            }
        </div>
    }
}
