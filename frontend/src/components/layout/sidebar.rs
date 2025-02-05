use super::logo::Logo;
use crate::components::selectors::benchmark_selector::BenchmarkSelector;
use crate::components::selectors::gitref_selector::GitrefSelector;
use crate::components::selectors::hardware_selector::HardwareSelector;
use crate::components::selectors::view_mode_selector::ViewModeSelector;
use crate::state::benchmark::{use_benchmark, BenchmarkAction};
use crate::state::gitref::use_gitref;
use crate::state::view_mode::{use_view_mode, ViewMode};
use iggy_bench_report::benchmark_kind::BenchmarkKind;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BenchmarkTab {
    Pinned,
    Balanced,
    EndToEnd,
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub on_gitref_select: Callback<String>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let gitref_ctx = use_gitref();
    let view_mode_ctx = use_view_mode();
    let benchmark_ctx = use_benchmark();
    let is_trend_view = matches!(view_mode_ctx.mode, ViewMode::GitrefTrend);
    let active_tab = use_state(|| BenchmarkTab::Pinned);

    let has_pinned_benchmarks = benchmark_ctx.state.entries.values().any(|benchmarks| {
        benchmarks.iter().any(|b| {
            matches!(
                b.params.benchmark_kind,
                BenchmarkKind::PinnedProducer
                    | BenchmarkKind::PinnedConsumer
                    | BenchmarkKind::PinnedProducerAndConsumer
            )
        })
    });

    let has_balanced_benchmarks = benchmark_ctx.state.entries.values().any(|benchmarks| {
        benchmarks.iter().any(|b| {
            matches!(
                b.params.benchmark_kind,
                BenchmarkKind::BalancedProducer
                    | BenchmarkKind::BalancedConsumerGroup
                    | BenchmarkKind::BalancedProducerAndConsumerGroup
            )
        })
    });

    let has_end_to_end_benchmarks = benchmark_ctx.state.entries.values().any(|benchmarks| {
        benchmarks.iter().any(|b| {
            matches!(
                b.params.benchmark_kind,
                BenchmarkKind::EndToEndProducingConsumer
                    | BenchmarkKind::EndToEndProducingConsumerGroup
            )
        })
    });

    let pinned_benchmark_count = benchmark_ctx
        .state
        .entries
        .values()
        .map(|benchmarks| {
            benchmarks
                .iter()
                .filter(|b| {
                    matches!(
                        b.params.benchmark_kind,
                        BenchmarkKind::PinnedProducer
                            | BenchmarkKind::PinnedConsumer
                            | BenchmarkKind::PinnedProducerAndConsumer
                    )
                })
                .count()
        })
        .sum::<usize>();

    let balanced_benchmark_count = benchmark_ctx
        .state
        .entries
        .values()
        .map(|benchmarks| {
            benchmarks
                .iter()
                .filter(|b| {
                    matches!(
                        b.params.benchmark_kind,
                        BenchmarkKind::BalancedProducer
                            | BenchmarkKind::BalancedConsumerGroup
                            | BenchmarkKind::BalancedProducerAndConsumerGroup
                    )
                })
                .count()
        })
        .sum::<usize>();

    let end_to_end_benchmark_count = benchmark_ctx
        .state
        .entries
        .values()
        .map(|benchmarks| {
            benchmarks
                .iter()
                .filter(|b| {
                    matches!(
                        b.params.benchmark_kind,
                        BenchmarkKind::EndToEndProducingConsumer
                            | BenchmarkKind::EndToEndProducingConsumerGroup
                    )
                })
                .count()
        })
        .sum::<usize>();

    fn get_default_kind_for_tab(tab: &BenchmarkTab) -> BenchmarkKind {
        match tab {
            BenchmarkTab::Pinned => BenchmarkKind::PinnedProducer,
            BenchmarkTab::Balanced => BenchmarkKind::BalancedProducer,
            BenchmarkTab::EndToEnd => BenchmarkKind::EndToEndProducingConsumer,
        }
    }

    // Switch to another available tab if current becomes unavailable
    {
        let active_tab = active_tab.clone();
        let benchmark_ctx = benchmark_ctx.clone();
        use_effect_with(
            (
                has_pinned_benchmarks,
                has_balanced_benchmarks,
                has_end_to_end_benchmarks,
            ),
            move |(has_pinned, has_balanced, has_end_to_end)| {
                let cleanup = || ();

                let current_unavailable = match *active_tab {
                    BenchmarkTab::Pinned => !has_pinned,
                    BenchmarkTab::Balanced => !has_balanced,
                    BenchmarkTab::EndToEnd => !has_end_to_end,
                };

                if current_unavailable {
                    // Try to switch to first available tab
                    if *has_pinned {
                        active_tab.set(BenchmarkTab::Pinned);
                        benchmark_ctx
                            .dispatch
                            .emit(BenchmarkAction::SelectBenchmarkKind(
                                get_default_kind_for_tab(&BenchmarkTab::Pinned),
                            ));
                    } else if *has_balanced {
                        active_tab.set(BenchmarkTab::Balanced);
                        benchmark_ctx
                            .dispatch
                            .emit(BenchmarkAction::SelectBenchmarkKind(
                                get_default_kind_for_tab(&BenchmarkTab::Balanced),
                            ));
                    } else if *has_end_to_end {
                        active_tab.set(BenchmarkTab::EndToEnd);
                        benchmark_ctx
                            .dispatch
                            .emit(BenchmarkAction::SelectBenchmarkKind(
                                get_default_kind_for_tab(&BenchmarkTab::EndToEnd),
                            ));
                    }
                }

                cleanup
            },
        );
    }

    let on_tab_click = {
        let active_tab = active_tab.clone();
        let benchmark_ctx = benchmark_ctx.clone();
        Callback::from(move |tab: BenchmarkTab| {
            let should_switch = match tab {
                BenchmarkTab::Pinned => has_pinned_benchmarks,
                BenchmarkTab::Balanced => has_balanced_benchmarks,
                BenchmarkTab::EndToEnd => has_end_to_end_benchmarks,
            };

            if should_switch {
                benchmark_ctx
                    .dispatch
                    .emit(BenchmarkAction::SelectBenchmarkKind(
                        get_default_kind_for_tab(&tab),
                    ));
                active_tab.set(tab);
            }
        })
    };

    html! {
        <div class="sidebar">
            <Logo />
            <HardwareSelector />
            <ViewModeSelector />

            if !is_trend_view {
                <GitrefSelector
                    gitrefs={gitref_ctx.state.gitrefs.clone()}
                    selected_gitref={gitref_ctx.state.selected_gitref.clone().unwrap_or_default()}
                    on_gitref_select={props.on_gitref_select.clone()}
                />
            }

            <h3>{"Benchmarks"}</h3>
            <div class="sidebar-tabs">
                <div class="tab-list">
                    <button
                        class={classes!(
                            "tab-button",
                            (*active_tab == BenchmarkTab::Pinned).then_some("active"),
                            (!has_pinned_benchmarks).then_some("inactive")
                        )}
                        disabled={!has_pinned_benchmarks}
                        onclick={let on_tab_click = on_tab_click.clone();
                                move |_| on_tab_click.emit(BenchmarkTab::Pinned)}
                    >
                        { "Pinned ("}{pinned_benchmark_count}{")"}
                    </button>
                    <button
                        class={classes!(
                            "tab-button",
                            (*active_tab == BenchmarkTab::Balanced).then_some("active"),
                            (!has_balanced_benchmarks).then_some("inactive")
                        )}
                        disabled={!has_balanced_benchmarks}
                        onclick={let on_tab_click = on_tab_click.clone();
                                move |_| on_tab_click.emit(BenchmarkTab::Balanced)}
                    >
                        { "Balanced ("}{balanced_benchmark_count}{")"}
                    </button>
                    <button
                        class={classes!(
                            "tab-button",
                            (*active_tab == BenchmarkTab::EndToEnd).then_some("active"),
                            (!has_end_to_end_benchmarks).then_some("inactive")
                        )}
                        disabled={!has_end_to_end_benchmarks}
                        onclick={let on_tab_click = on_tab_click.clone();
                                move |_| on_tab_click.emit(BenchmarkTab::EndToEnd)}
                    >
                        { "End to End ("}{end_to_end_benchmark_count}{")"}
                    </button>
                </div>
                <div class={classes!(
                    "tab-content",
                    (*active_tab == BenchmarkTab::Pinned).then_some("active")
                )}>
                    <BenchmarkSelector kind={get_default_kind_for_tab(&BenchmarkTab::Pinned)} />
                </div>
                <div class={classes!(
                    "tab-content",
                    (*active_tab == BenchmarkTab::Balanced).then_some("active")
                )}>
                    <BenchmarkSelector kind={get_default_kind_for_tab(&BenchmarkTab::Balanced)} />
                </div>
                <div class={classes!(
                    "tab-content",
                    (*active_tab == BenchmarkTab::EndToEnd).then_some("active")
                )}>
                    <BenchmarkSelector kind={get_default_kind_for_tab(&BenchmarkTab::EndToEnd)} />
                </div>
            </div>
        </div>
    }
}
