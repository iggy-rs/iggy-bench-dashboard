use crate::state::ui::{use_ui, UiAction, ViewMode};
use yew::prelude::*;

#[function_component(ViewModeSelector)]
pub fn view_mode_toggle() -> Html {
    let ui_state = use_ui();
    let is_trend_view = matches!(ui_state.view_mode, ViewMode::GitrefTrend);

    let onclick = {
        let ui_state = ui_state.clone();
        Callback::from(move |_| {
            ui_state.dispatch(UiAction::SetViewMode(if is_trend_view {
                ViewMode::SingleGitref
            } else {
                ViewMode::GitrefTrend
            }));
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
