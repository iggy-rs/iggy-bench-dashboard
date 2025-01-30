use super::selectors::measurement_type_selector::MeasurementType;
use crate::{
    api,
    components::layout::{main_content::MainContent, sidebar::Sidebar},
    state::{
        benchmark::{use_benchmark, BenchmarkAction, BenchmarkContext},
        gitref::{use_gitref, GitrefAction, GitrefContext},
        hardware::{use_hardware, HardwareAction, HardwareContext},
        view_mode::use_view_mode,
    },
};
use gloo::console::log;
use yew::prelude::*;

// Props definitions
#[derive(Properties, PartialEq)]
pub struct AppContentProps {
    pub selected_measurement: MeasurementType,
    pub is_benchmark_tooltip_visible: bool,
    pub on_measurement_select: Callback<MeasurementType>,
    pub on_benchmark_tooltip_toggle: Callback<()>,
}

// Hardware initialization hook
#[hook]
fn use_init_hardware(hardware_ctx: HardwareContext) {
    use_effect_with((), move |_| {
        let dispatch = hardware_ctx.dispatch.clone();
        yew::platform::spawn_local(async move {
            match api::fetch_hardware_configurations().await {
                Ok(hardware) => {
                    if !hardware.is_empty() {
                        dispatch.emit(HardwareAction::SetHardwareList(hardware.clone()));
                        let default_hardware = &hardware[0];
                        dispatch.emit(HardwareAction::SelectHardware(
                            default_hardware.identifier.clone(),
                        ));
                    }
                }
                Err(e) => log!(format!("Error fetching hardware: {}", e)),
            }
        });
        || ()
    });
}

// Gitref loading hook
#[hook]
fn use_load_gitrefs(
    gitref_ctx: GitrefContext,
    benchmark_ctx: BenchmarkContext,
    hardware: Option<String>,
) {
    use_effect_with(hardware.clone(), move |hardware| {
        let gitref_ctx = gitref_ctx.clone();
        let benchmark_ctx = benchmark_ctx.clone();
        let hardware = hardware.clone();

        if let Some(hardware) = hardware {
            yew::platform::spawn_local(async move {
                match api::fetch_gitrefs_for_hardware(&hardware).await {
                    Ok(vers) => {
                        gitref_ctx
                            .dispatch
                            .emit(GitrefAction::SetGitrefs(vers.clone()));

                        if !vers.is_empty() {
                            // Always use the first version when switching hardware
                            // since benchmarks are hardware-specific
                            let selected_gitref = vers[0].clone();
                            log!("Using first available version for new hardware");

                            gitref_ctx
                                .dispatch
                                .emit(GitrefAction::SetSelectedGitref(Some(
                                    selected_gitref.clone(),
                                )));

                            // Always fetch benchmarks for new hardware
                            match api::fetch_benchmarks_for_hardware_and_gitref(
                                &hardware,
                                &selected_gitref,
                            )
                            .await
                            {
                                Ok(benchmarks) => {
                                    benchmark_ctx
                                        .dispatch
                                        .emit(BenchmarkAction::SetBenchmarksForGitref(benchmarks));
                                }
                                Err(e) => log!(format!("Error fetching benchmarks: {}", e)),
                            }
                        }
                    }
                    Err(e) => log!(format!("Error fetching git refs: {}", e)),
                }
            });
        }
        || ()
    });
}

#[hook]
fn use_load_benchmarks(
    benchmark_ctx: BenchmarkContext,
    hardware: Option<String>,
    gitref: Option<String>,
    is_hardware_changing: bool,
) {
    use_effect_with(
        (hardware.clone(), gitref.clone(), is_hardware_changing),
        move |(hardware, gitref, is_hardware_changing)| {
            let benchmark_ctx = benchmark_ctx.clone();
            let hardware = hardware.clone();
            let gitref = gitref.clone();

            // Only load benchmarks when gitref changes explicitly (not during hardware change)
            // because of god damn race conditions.
            if !is_hardware_changing {
                if let (Some(hardware), Some(gitref)) = (hardware, gitref) {
                    yew::platform::spawn_local(async move {
                        match api::fetch_benchmarks_for_hardware_and_gitref(&hardware, &gitref)
                            .await
                        {
                            Ok(benchmarks) => {
                                benchmark_ctx
                                    .dispatch
                                    .emit(BenchmarkAction::SetBenchmarksForGitref(benchmarks));
                            }
                            Err(e) => log!(format!("Error fetching benchmarks: {}", e)),
                        }
                    });
                }
            }
            || ()
        },
    );
}

#[function_component(AppContent)]
pub fn app_content(props: &AppContentProps) -> Html {
    let hardware_ctx = use_hardware();
    let gitref_ctx = use_gitref();
    let benchmark_ctx = use_benchmark();
    let view_mode_ctx = use_view_mode();
    let is_hardware_changing = use_state(|| false);

    // Get theme context
    let theme_ctx = use_context::<(bool, Callback<()>)>().expect("Theme context not found");
    let is_dark = theme_ctx.0;
    let theme_toggle = theme_ctx.1;

    // Create a new callback that adapts () -> bool
    let on_theme_toggle = {
        let theme_toggle = theme_toggle.clone();
        Callback::from(move |_: bool| {
            theme_toggle.emit(());
        })
    };

    // Initialize data loading hooks
    use_init_hardware(hardware_ctx.clone());

    {
        let is_hardware_changing = is_hardware_changing.clone();
        use_effect_with(hardware_ctx.state.selected_hardware.clone(), move |_| {
            is_hardware_changing.set(true);
            || ()
        });
    }

    use_load_gitrefs(
        gitref_ctx.clone(),
        benchmark_ctx.clone(),
        hardware_ctx.state.selected_hardware.clone(),
    );

    {
        let is_hardware_changing = is_hardware_changing.clone();
        use_effect_with(gitref_ctx.state.selected_gitref.clone(), move |_| {
            is_hardware_changing.set(false);
            || ()
        });
    }

    use_load_benchmarks(
        benchmark_ctx.clone(),
        hardware_ctx.state.selected_hardware.clone(),
        gitref_ctx.state.selected_gitref.clone(),
        *is_hardware_changing,
    );

    html! {
        <div class="container">
            <Sidebar
                on_gitref_select={Callback::from(move |gitref: String| {
                    gitref_ctx.dispatch.emit(GitrefAction::SetSelectedGitref(Some(gitref)));
                })}
            />
            <MainContent
                selected_measurement={props.selected_measurement.clone()}
                selected_gitref={gitref_ctx.state.selected_gitref.clone().unwrap_or_default()}
                is_dark={is_dark}
                is_benchmark_tooltip_visible={props.is_benchmark_tooltip_visible}
                on_theme_toggle={on_theme_toggle}
                on_benchmark_tooltip_toggle={props.on_benchmark_tooltip_toggle.clone()}
                on_measurement_select={props.on_measurement_select.clone()}
                view_mode={view_mode_ctx.mode.clone()}
            />
        </div>
    }
}
