mod api;
mod components;
mod config;
mod error;
mod state;

use crate::{
    components::{app_content::AppContent, footer::Footer, theme_provider::ThemeProvider},
    state::{hardware::HardwareProvider, view_mode::ViewModeProvider},
};
use components::selectors::measurement_type_selector::MeasurementType;
use state::{benchmark::BenchmarkProvider, gitref::GitrefProvider};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let selected_measurement = use_state(|| MeasurementType::Latency);
    let is_benchmark_tooltip_visible = use_state(|| false);

    html! {
        <ThemeProvider>
            <div class="app-container">
                <HardwareProvider>
                    <GitrefProvider>
                        <BenchmarkProvider>
                            <ViewModeProvider>
                                <AppContent
                                    selected_measurement={(*selected_measurement).clone()}
                                    on_measurement_select={Callback::from(move |measurement_type| {
                                        selected_measurement.set(measurement_type);
                                    })}
                                    is_benchmark_tooltip_visible={*is_benchmark_tooltip_visible}
                                    on_benchmark_tooltip_toggle={Callback::from(move |()| {
                                        let current = *is_benchmark_tooltip_visible;
                                        is_benchmark_tooltip_visible.set(!current);
                                    })}
                                />
                            </ViewModeProvider>
                        </BenchmarkProvider>
                    </GitrefProvider>
                </HardwareProvider>
                <Footer />
            </div>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
