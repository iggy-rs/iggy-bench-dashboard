use yew::prelude::*;

#[function_component(Logo)]
pub fn logo() -> Html {
    html! {
        <div class="logo">
            <img src="/assets/iggy.png" alt="Iggy Logo" />
            <h1>{"Iggy Benchmarks"}</h1>
        </div>
    }
}
