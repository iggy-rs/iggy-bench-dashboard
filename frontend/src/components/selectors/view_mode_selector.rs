use crate::state::view_mode::{use_view_mode, ViewMode, ViewModeAction};
use yew::prelude::*;

#[function_component(ViewModeSelector)]
pub fn view_mode_toggle() -> Html {
    let view_mode_ctx = use_view_mode();
    let is_trend_view = matches!(view_mode_ctx.mode, ViewMode::GitrefTrend);

    let onclick = {
        let view_mode_ctx = view_mode_ctx.clone();
        Callback::from(move |_| {
            view_mode_ctx.dispatch(ViewModeAction::ToggleMode);
        })
    };

    html! {
        <div class="view-mode-container">
            <h3>{"View Mode"}</h3>
            <div class="segmented-control">
                <button
                    class={if !is_trend_view { "segment active" } else { "segment" }}
                    onclick={onclick.clone()}
                >
                    {"Single"}
                </button>
                <button
                    class={if is_trend_view { "segment active" } else { "segment" }}
                    {onclick}
                >
                    {"Trend"}
                </button>
            </div>
        </div>
    }
}
