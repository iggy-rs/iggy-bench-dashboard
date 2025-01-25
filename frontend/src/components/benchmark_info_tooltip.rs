use crate::state::view_mode::ViewMode;
use gloo::timers::callback::Timeout;
use shared::BenchmarkReportLight;
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BenchmarkInfoTooltipProps {
    pub benchmark_report: Option<BenchmarkReportLight>,
    pub visible: bool,
    pub view_mode: ViewMode,
}

#[function_component(BenchmarkInfoTooltip)]
pub fn benchmark_info_tooltip(props: &BenchmarkInfoTooltipProps) -> Html {
    if !props.visible || props.benchmark_report.is_none() {
        return html! {};
    }

    let benchmark_report = props.benchmark_report.as_ref().unwrap();
    let hardware = &benchmark_report.hardware;
    let params = &benchmark_report.params;
    let is_trend_view = matches!(props.view_mode, ViewMode::GitrefTrend);
    let notification_visible = use_state(|| false);

    let onclick = {
        let command = params.bench_command.clone();
        let notification_visible = notification_visible.clone();
        Callback::from(move |_| {
            if let Some(window) = window() {
                let clipboard = window.navigator().clipboard();
                let _ = clipboard.write_text(&command);
                notification_visible.set(true);

                // Hide notification after 1 second
                let notification_visible = notification_visible.clone();
                let timeout = Timeout::new(1_000, move || {
                    notification_visible.set(false);
                });
                timeout.forget();
            }
        })
    };

    html! {
        <div class="benchmark-info-tooltip">
            <div class="tooltip-section">
                <h4>{"Hardware"}</h4>
                <div class="tooltip-content">
                    <p><strong>{"CPU: "}</strong>{&hardware.cpu_name}</p>
                    <p><strong>{"Cores: "}</strong>{hardware.cpu_cores}</p>
                    <p><strong>{"Memory: "}</strong>{hardware.total_memory_mb}{" MB"}</p>
                    <p><strong>{"OS: "}</strong>{format!("{} {}", hardware.os_name, hardware.os_version)}</p>
                </div>
            </div>
            <div class="tooltip-section">
                <h4>{"Benchmark Parameters"}</h4>
                <div class="tooltip-content">
                    {if !is_trend_view {
                        html! {
                            <p><strong>{"Time: "}</strong>{&benchmark_report.timestamp}</p>
                        }
                    } else {
                        html! {}
                    }}
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
                    {if !is_trend_view {
                        html! {
                            <>
                                <p><strong>{"Git ref: "}</strong>{params.gitref.clone()}</p>
                                <p><strong>{"Git ref date: "}</strong>{params.gitref_date.clone()}</p>
                            </>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
            <div class="tooltip-section">
                <h4>{"Command"}</h4>
                <div class="tooltip-content">
                    <p class="command-row">
                        <span class="command-text">{&params.bench_command}</span>
                        <span class="copy-button-container">
                            <button onclick={onclick} class="copy-button">{"Copy"}</button>
                            <span class={classes!(
                                "copy-notification",
                                (*notification_visible).then_some("visible")
                            )}>
                                {"Copied!"}
                            </span>
                        </span>
                    </p>
                </div>
            </div>
        </div>
    }
}
