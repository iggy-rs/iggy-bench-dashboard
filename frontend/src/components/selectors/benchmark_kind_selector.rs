use iggy_benchmark_report::benchmark_kind::BenchmarkKind;
use std::collections::HashSet;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BenchmarkKindSelectorProps {
    pub selected_kind: BenchmarkKind,
    pub on_kind_select: Callback<BenchmarkKind>,
    pub is_consumer_group: bool,
    pub available_kinds: HashSet<BenchmarkKind>,
}

#[function_component(BenchmarkKindSelector)]
pub fn benchmark_kind_selector(props: &BenchmarkKindSelectorProps) -> Html {
    html! {
        <div class="benchmark-grid">
            if !props.is_consumer_group {
                <>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::Send).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::Send)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::Send)
                        }
                    >
                        <span class="benchmark-option-icon">{"↑"}</span>
                        <span class="benchmark-option-label">{"Send"}</span>
                    </button>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::Poll).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::Poll)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::Poll)
                        }
                    >
                        <span class="benchmark-option-icon">{"↓"}</span>
                        <span class="benchmark-option-label">{"Poll"}</span>
                    </button>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::SendAndPoll).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::SendAndPoll)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::SendAndPoll)
                        }
                    >
                        <span class="benchmark-option-icon">{"⇅"}</span>
                        <span class="benchmark-option-label">{"Send & Poll"}</span>
                    </button>
                </>
            } else {
                <>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::ConsumerGroupSend).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::ConsumerGroupSend)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::ConsumerGroupSend)
                        }
                    >
                        <span class="benchmark-option-icon">{"↑"}</span>
                        <span class="benchmark-option-label">{"Send"}</span>
                    </button>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::ConsumerGroupPoll).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::ConsumerGroupPoll)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::ConsumerGroupPoll)
                        }
                    >
                        <span class="benchmark-option-icon">{"↓"}</span>
                        <span class="benchmark-option-label">{"Poll"}</span>
                    </button>
                    <button
                        class={classes!(
                            "benchmark-option",
                            matches!(props.selected_kind, BenchmarkKind::ConsumerGroupSendAndPoll).then_some("active"),
                            (!props.available_kinds.contains(&BenchmarkKind::ConsumerGroupSendAndPoll)).then_some("inactive")
                        )}
                        onclick={
                            let on_kind_select = props.on_kind_select.clone();
                            move |_| on_kind_select.emit(BenchmarkKind::ConsumerGroupSendAndPoll)
                        }
                    >
                        <span class="benchmark-option-icon">{"⇅"}</span>
                        <span class="benchmark-option-label">{"Send & Poll"}</span>
                    </button>
                </>
            }
        </div>
    }
}
