use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ServerStatsToggleProps {
    pub on_toggle: Callback<()>,
    pub is_visible: bool,
}

#[function_component(ServerStatsToggle)]
pub fn server_stats_toggle(props: &ServerStatsToggleProps) -> Html {
    let onclick = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_| {
            on_toggle.emit(());
        })
    };

    html! {
        <button
            class={classes!("server-stats-button", props.is_visible.then_some("active"))}
            {onclick}
            title="View Server Stats"
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
                <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
                <line x1="6" y1="6" x2="6.01" y2="6"></line>
                <line x1="6" y1="18" x2="6.01" y2="18"></line>
            </svg>
        </button>
    }
}
