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

    // Create context
    let theme_context = use_state(|| *is_dark);
    let toggle_theme = {
        let is_dark = is_dark.clone();
        let theme_context = theme_context.clone();
        Callback::from(move |_| {
            let new_value = !*is_dark;
            is_dark.set(new_value);
            theme_context.set(new_value);
        })
    };

    html! {
        <ContextProvider<(bool, Callback<()>)> context={(*theme_context, toggle_theme)}>
            {props.children.clone()}
        </ContextProvider<(bool, Callback<()>)>>
    }
}
