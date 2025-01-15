use crate::components::benchmark_info_toggle::BenchmarkInfoToggle;
use crate::components::benchmark_info_tooltip::BenchmarkInfoTooltip;
use crate::components::theme_toggle::ThemeToggle;
use crate::state::benchmark::use_benchmark;
use crate::state::hardware::use_hardware;
use crate::state::view_mode::use_view_mode;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TopBarProps {
    pub is_dark: bool,
    pub is_benchmark_info_visible: bool,
    pub selected_version: String,
    pub on_theme_toggle: Callback<bool>,
    pub on_benchmark_info_toggle: Callback<()>,
}

#[function_component(TopBar)]
pub fn topbar(props: &TopBarProps) -> Html {
    let hardware_ctx = use_hardware();
    let benchmark_ctx = use_benchmark();
    let view_mode_ctx = use_view_mode();

    html! {
        <div class="top-buttons">
            <div class="controls">
                <ThemeToggle
                    is_dark={props.is_dark}
                    on_toggle={props.on_theme_toggle.clone()}
                />
                {
                    if let Some(selected_benchmark) = &benchmark_ctx.state.selected_benchmark {
                        if !props.selected_version.is_empty() {
                            let hardware = hardware_ctx.state.selected_hardware.clone().unwrap_or_default();
                            let benchmark_path = format!("{}_{}_{}",
                                selected_benchmark,
                                props.selected_version,
                                hardware
                            );
                            let data_json_path = format!("/performance_results/{}/data.json", benchmark_path);
                            html! {
                                <>
                                    <a class="download-button" href={data_json_path} download={format!("{}-data.json", benchmark_path)} title="Download Raw Data">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                                            <polyline points="7 10 12 15 17 10"/>
                                            <line x1="12" y1="15" x2="12" y2="3"/>
                                        </svg>
                                    </a>
                                    <div class="info-container">
                                        <BenchmarkInfoToggle
                                            is_visible={props.is_benchmark_info_visible}
                                            on_toggle={props.on_benchmark_info_toggle.clone()}
                                        />
                                        {
                                            if props.is_benchmark_info_visible && benchmark_ctx.state.benchmark_info.is_some() {
                                                html! {
                                                    <BenchmarkInfoTooltip
                                                        benchmark_info={benchmark_ctx.state.benchmark_info.clone()}
                                                        visible={true}
                                                        view_mode={view_mode_ctx.mode.clone()}
                                                    />
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </>
                            }
                        } else {
                            html! {}
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
