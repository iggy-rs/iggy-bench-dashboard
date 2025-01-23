use gloo::{
    storage::{LocalStorage, Storage},
    utils::document,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    pub children: Html,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let is_dark = use_state(|| {
        LocalStorage::get("theme")
            .map(|theme: String| theme == "dark")
            .unwrap_or(false)
    });

    // Effect to update body class and local storage when theme changes
    {
        let is_dark = is_dark.clone();
        use_effect_with(*is_dark, move |is_dark| {
            let body = document().body().unwrap();
            if *is_dark {
                body.set_class_name("dark");
            } else {
                body.set_class_name("");
            }

            let _ = LocalStorage::set("theme", if *is_dark { "dark" } else { "light" });

            || ()
        });
    }

    let toggle_theme = {
        let is_dark = is_dark.clone();
        Callback::from(move |_| {
            is_dark.set(!*is_dark);
        })
    };

    html! {
        <ContextProvider<(bool, Callback<()>)> context={(*is_dark, toggle_theme)}>
            {props.children.clone()}
        </ContextProvider<(bool, Callback<()>)>>
    }
}
