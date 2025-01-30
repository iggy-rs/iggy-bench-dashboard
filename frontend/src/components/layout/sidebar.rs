use super::logo::Logo;
use crate::components::selectors::benchmark_selector::BenchmarkSelector;
use crate::components::selectors::gitref_selector::GitrefSelector;
use crate::components::selectors::hardware_selector::HardwareSelector;
use crate::components::view_mode_toggle::ViewModeToggle;
use crate::state::benchmark::{use_benchmark, BenchmarkAction};
use crate::state::gitref::use_gitref;
use crate::state::view_mode::{use_view_mode, ViewMode};
use iggy_benchmark_report::benchmark_kind::BenchmarkKind;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BenchmarkTab {
    Regular,
    ConsumerGroup,
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
    let active_tab = use_state(|| BenchmarkTab::Regular);

    let has_regular_benchmarks = benchmark_ctx.state.entries.values().any(|benchmarks| {
        benchmarks.iter().any(|b| {
            matches!(
                b.params.benchmark_kind,
                BenchmarkKind::Send | BenchmarkKind::Poll | BenchmarkKind::SendAndPoll
            )
        })
    });

    let has_consumer_group_benchmarks = benchmark_ctx.state.entries.values().any(|benchmarks| {
        benchmarks.iter().any(|b| {
            matches!(
                b.params.benchmark_kind,
                BenchmarkKind::ConsumerGroupSend
                    | BenchmarkKind::ConsumerGroupPoll
                    | BenchmarkKind::ConsumerGroupSendAndPoll
            )
        })
    });

    // Switch to Regular tab if Consumer Group benchmarks become unavailable
    {
        let active_tab = active_tab.clone();
        let benchmark_ctx = benchmark_ctx.clone();
        use_effect_with(
            (has_regular_benchmarks, has_consumer_group_benchmarks),
            move |(has_regular, has_consumer_group)| {
                if *active_tab == BenchmarkTab::ConsumerGroup && !has_consumer_group && *has_regular
                {
                    // Switch to Regular tab
                    active_tab.set(BenchmarkTab::Regular);

                    // Find and select first available regular benchmark kind
                    let kinds = vec![
                        BenchmarkKind::Send,
                        BenchmarkKind::Poll,
                        BenchmarkKind::SendAndPoll,
                    ];
                    if let Some(kind) = kinds
                        .into_iter()
                        .find(|kind| benchmark_ctx.state.entries.contains_key(kind))
                    {
                        benchmark_ctx
                            .dispatch
                            .emit(BenchmarkAction::SelectBenchmarkKind(kind));
                    }
                }
                || ()
            },
        );
    }

    // Function to get the first available benchmark kind for a tab
    let get_first_available_kind = {
        let benchmark_ctx = benchmark_ctx.clone();
        move |is_consumer_group: bool| -> Option<BenchmarkKind> {
            let kinds = if is_consumer_group {
                vec![
                    BenchmarkKind::ConsumerGroupSend,
                    BenchmarkKind::ConsumerGroupPoll,
                    BenchmarkKind::ConsumerGroupSendAndPoll,
                ]
            } else {
                vec![
                    BenchmarkKind::Send,
                    BenchmarkKind::Poll,
                    BenchmarkKind::SendAndPoll,
                ]
            };

            kinds
                .into_iter()
                .find(|kind| benchmark_ctx.state.entries.contains_key(kind))
        }
    };

    let on_tab_click = {
        let active_tab = active_tab.clone();
        let benchmark_ctx = benchmark_ctx.clone();
        Callback::from(move |tab: BenchmarkTab| {
            match tab {
                BenchmarkTab::Regular if has_regular_benchmarks => {
                    if let Some(kind) = get_first_available_kind(false) {
                        benchmark_ctx
                            .dispatch
                            .emit(BenchmarkAction::SelectBenchmarkKind(kind));
                        active_tab.set(tab);
                    }
                }
                BenchmarkTab::ConsumerGroup if has_consumer_group_benchmarks => {
                    if let Some(kind) = get_first_available_kind(true) {
                        benchmark_ctx
                            .dispatch
                            .emit(BenchmarkAction::SelectBenchmarkKind(kind));
                        active_tab.set(tab);
                    }
                }
                _ => (), // Do nothing if the tab is inactive
            }
        })
    };

    html! {
        <div class="sidebar">
            <Logo />
            <HardwareSelector />
            <ViewModeToggle />

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
                            (*active_tab == BenchmarkTab::Regular).then_some("active"),
                            (!has_regular_benchmarks).then_some("inactive")
                        )}
                        disabled={!has_regular_benchmarks}
                        onclick={let on_tab_click = on_tab_click.clone();
                                move |_| on_tab_click.emit(BenchmarkTab::Regular)}
                    >
                        { "Regular" }
                    </button>
                    <button
                        class={classes!(
                            "tab-button",
                            (*active_tab == BenchmarkTab::ConsumerGroup).then_some("active"),
                            (!has_consumer_group_benchmarks).then_some("inactive")
                        )}
                        disabled={!has_consumer_group_benchmarks}
                        onclick={let on_tab_click = on_tab_click.clone();
                                move |_| on_tab_click.emit(BenchmarkTab::ConsumerGroup)}
                    >
                        { "Consumer Group" }
                    </button>
                </div>
                <div class={classes!(
                    "tab-content",
                    (*active_tab == BenchmarkTab::Regular).then_some("active")
                )}>
                    <BenchmarkSelector is_consumer_group={false} />
                </div>
                <div class={classes!(
                    "tab-content",
                    (*active_tab == BenchmarkTab::ConsumerGroup).then_some("active")
                )}>
                    <BenchmarkSelector is_consumer_group={true} />
                </div>
            </div>
        </div>
    }
}
