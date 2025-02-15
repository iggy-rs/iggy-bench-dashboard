mod api;
mod components;
mod config;
mod error;
mod router;
mod state;

use crate::{
    components::{app_content::AppContent, footer::Footer},
    state::hardware::HardwareProvider,
};
use components::theme::theme_provider::ThemeProvider;
use router::AppRoute;
use state::{benchmark::BenchmarkProvider, gitref::GitrefProvider, ui::UiProvider};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Single | AppRoute::Home => html! {
            <ThemeProvider>
                <UiProvider>
                    <div class="app-container">
                        <HardwareProvider>
                            <GitrefProvider>
                                <BenchmarkProvider>
                                    <AppContent />
                                </BenchmarkProvider>
                            </GitrefProvider>
                        </HardwareProvider>
                        <Footer />
                    </div>
                </UiProvider>
            </ThemeProvider>
        },
        AppRoute::NotFound => html! { "404 Not Found" },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
