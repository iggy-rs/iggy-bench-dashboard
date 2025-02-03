use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BenchmarkInfoToggleProps {
    pub on_toggle: Callback<()>,
    pub is_visible: bool,
}

#[function_component(BenchmarkInfoToggle)]
pub fn benchmark_info_toggle(props: &BenchmarkInfoToggleProps) -> Html {
    let onclick = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_| {
            on_toggle.emit(());
        })
    };

    html! {
        <button
            class={classes!("icon-button", props.is_visible.then_some("active"))}
            {onclick}
            title="Toggle Benchmark Info"
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                <line x1="12" y1="8" x2="12" y2="12"/>
                <line x1="12" y1="16" x2="12" y2="16"/>
            </svg>
        </button>
    }
}
