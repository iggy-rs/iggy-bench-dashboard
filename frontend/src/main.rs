mod api;
mod components;
mod config;
mod error;
mod state;
mod types;

use crate::{
    components::{app_content::AppContent, footer::Footer, theme_provider::ThemeProvider},
    state::{
        benchmark::BenchmarkProvider, gitref::VersionProvider, hardware::HardwareProvider,
        view_mode::ViewModeProvider,
    },
    types::MeasurementType,
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let selected_file = use_state(|| MeasurementType::Latency);
    let is_benchmark_info_visible = use_state(|| false);

    html! {
        <ThemeProvider>
            <div class="app-container">
                <HardwareProvider>
                    <VersionProvider>
                        <BenchmarkProvider>
                            <ViewModeProvider>
                                <AppContent
                                    selected_file={(*selected_file).clone()}
                                    is_dark={false} // This will be overridden by ThemeProvider context
                                    is_benchmark_info_visible={*is_benchmark_info_visible}
                                    on_file_select={Callback::from(move |measurement_type: MeasurementType| {
                                        selected_file.set(measurement_type);
                                    })}
                                    on_theme_toggle={Callback::from(|_| {})} // This will be overridden by ThemeProvider context
                                    on_benchmark_info_toggle={Callback::from(move |_| {
                                        let current = *is_benchmark_info_visible;
                                        is_benchmark_info_visible.set(!current);
                                    })}
                                />
                            </ViewModeProvider>
                        </BenchmarkProvider>
                    </VersionProvider>
                </HardwareProvider>
                <Footer />
            </div>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
