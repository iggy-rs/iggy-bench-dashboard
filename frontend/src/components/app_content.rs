use crate::{
    api,
    components::layout::{main_content::MainContent, sidebar::Sidebar, topbar::TopBar},
    state::{
        benchmark::{use_benchmark, BenchmarkAction, BenchmarkContext},
        gitref::{use_version, VersionAction, VersionContext},
        hardware::{use_hardware, HardwareAction, HardwareContext},
        view_mode::{use_view_mode, ViewMode},
    },
    types::MeasurementType,
};
use gloo::console::log;
use yew::prelude::*;

// Props definitions
#[derive(Properties, PartialEq)]
pub struct AppContentProps {
    pub selected_file: MeasurementType,
    pub is_dark: bool,
    pub is_benchmark_info_visible: bool,
    pub on_file_select: Callback<MeasurementType>,
    pub on_theme_toggle: Callback<bool>,
    pub on_benchmark_info_toggle: Callback<()>,
}

// Hardware initialization hook
#[hook]
fn use_init_hardware(hardware_ctx: HardwareContext) {
    use_effect_with((), move |_| {
        let dispatch = hardware_ctx.dispatch.clone();
        yew::platform::spawn_local(async move {
            match api::fetch_available_hardware().await {
                Ok(hardware) => {
                    if !hardware.is_empty() {
                        dispatch.emit(HardwareAction::SetHardwareList(hardware.clone()));
                        let default_hardware = hardware
                            .iter()
                            .find(|h| h.hostname == "atlas2")
                            .unwrap_or(&hardware[0]);
                        dispatch.emit(HardwareAction::SelectHardware(
                            default_hardware.hostname.clone(),
                        ));
                    }
                }
                Err(e) => log!(format!("Error fetching hardware: {}", e)),
            }
        });
        || ()
    });
}

// Version loading hook
#[hook]
fn use_load_versions(version_ctx: VersionContext, hardware: Option<String>) {
    use_effect_with(hardware.clone(), move |hardware| {
        let version_ctx = version_ctx.clone();
        let hardware = hardware.clone();

        if let Some(hardware) = hardware {
            yew::platform::spawn_local(async move {
                match api::fetch_versions_for_hardware(&hardware).await {
                    Ok(vers) => {
                        version_ctx
                            .dispatch
                            .emit(VersionAction::SetVersions(vers.clone()));
                        if !vers.is_empty() {
                            version_ctx
                                .dispatch
                                .emit(VersionAction::SetSelectedVersion(Some(vers[0].clone())));
                        }
                    }
                    Err(e) => log!(format!("Error fetching versions: {}", e)),
                }
            });
        }
        || ()
    });
}

// Benchmark loading hook
#[hook]
fn use_load_benchmarks(
    benchmark_ctx: BenchmarkContext,
    hardware: Option<String>,
    version: Option<String>,
) {
    use_effect_with(
        (hardware.clone(), version.clone()),
        move |(hardware, version)| {
            let benchmark_ctx = benchmark_ctx.clone();
            let current_benchmark = benchmark_ctx.state.selected_benchmark.clone();
            let hardware = hardware.clone();
            let version = version.clone();

            if let (Some(hardware), Some(version)) = (hardware, version) {
                yew::platform::spawn_local(async move {
                    match api::fetch_unique_benchmarks(Some(&version), Some(&hardware)).await {
                        Ok(benchmarks) => {
                            let should_keep_selection = current_benchmark
                                .as_ref()
                                .map(|current| benchmarks.iter().any(|b| b.name == *current))
                                .unwrap_or(false);

                            benchmark_ctx
                                .dispatch
                                .emit(BenchmarkAction::SetBenchmarks(benchmarks));

                            if should_keep_selection {
                                if let Some(benchmark) = current_benchmark {
                                    benchmark_ctx
                                        .dispatch
                                        .emit(BenchmarkAction::SelectBenchmark(benchmark));
                                }
                            }
                        }
                        Err(e) => log!(format!("Error fetching benchmarks: {}", e)),
                    }
                });
            }
            || ()
        },
    );
}

// Benchmark info loading hook
#[hook]
fn use_load_benchmark_info(
    benchmark_ctx: BenchmarkContext,
    selected_benchmark: Option<String>,
    hardware: Option<String>,
    version: Option<String>,
) {
    use_effect_with(
        (
            selected_benchmark.clone(),
            hardware.clone(),
            version.clone(),
        ),
        move |(selected_benchmark, hardware, version)| {
            let benchmark_ctx = benchmark_ctx.clone();
            let selected_benchmark = selected_benchmark.clone();
            let hardware = hardware.clone();
            let version = version.clone();

            if let (Some(benchmark_name), Some(hardware), Some(version)) =
                (selected_benchmark, hardware, version)
            {
                yew::platform::spawn_local(async move {
                    let benchmark_path = format!("{}_{}_{}", benchmark_name, version, hardware);
                    match api::fetch_benchmark_info(&benchmark_path).await {
                        Ok(info) => {
                            benchmark_ctx
                                .dispatch
                                .emit(BenchmarkAction::SetBenchmarkInfo(Box::new(Some(info))));
                        }
                        Err(e) => {
                            log!(format!("Error fetching benchmark info: {}", e));
                            benchmark_ctx
                                .dispatch
                                .emit(BenchmarkAction::SetBenchmarkInfo(Box::new(None)));
                        }
                    }
                });
            }
            || ()
        },
    );
}

#[function_component(AppContent)]
pub fn app_content(props: &AppContentProps) -> Html {
    let hardware_ctx = use_hardware();
    let version_ctx = use_version();
    let benchmark_ctx = use_benchmark();
    let view_mode_ctx = use_view_mode();

    // Get theme context
    let theme_ctx = use_context::<(bool, Callback<()>)>().expect("Theme context not found");
    let is_dark = theme_ctx.0;
    let on_theme_toggle = theme_ctx.1;

    // Initialize data loading hooks
    use_init_hardware(hardware_ctx.clone());
    use_load_versions(
        version_ctx.clone(),
        hardware_ctx.state.selected_hardware.clone(),
    );
    use_load_benchmarks(
        benchmark_ctx.clone(),
        hardware_ctx.state.selected_hardware.clone(),
        version_ctx.state.selected_version.clone(),
    );
    use_load_benchmark_info(
        benchmark_ctx.clone(),
        benchmark_ctx.state.selected_benchmark.clone(),
        hardware_ctx.state.selected_hardware.clone(),
        version_ctx.state.selected_version.clone(),
    );

    html! {
        <div class="container">
            <Sidebar
                selected_file={props.selected_file.clone()}
                on_file_select={props.on_file_select.clone()}
                on_version_select={Callback::from(move |version: String| {
                    version_ctx.dispatch.emit(VersionAction::SetSelectedVersion(Some(version)));
                })}
            />
            <div class="content">
                <TopBar
                    is_dark={is_dark}
                    is_benchmark_info_visible={props.is_benchmark_info_visible}
                    selected_version={version_ctx.state.selected_version.clone().unwrap_or_default()}
                    on_theme_toggle={Callback::from(move |_| on_theme_toggle.emit(()))}
                    on_benchmark_info_toggle={props.on_benchmark_info_toggle.clone()}
                />
                <MainContent
                    selected_file={props.selected_file.clone()}
                    selected_version={if matches!(view_mode_ctx.mode, ViewMode::VersionTrend) {
                        String::new()
                    } else {
                        version_ctx.state.selected_version.clone().unwrap_or_default()
                    }}
                    is_dark={is_dark}
                />
            </div>
        </div>
    }
}
