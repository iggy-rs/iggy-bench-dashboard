use yew::prelude::*;

#[function_component(Logo)]
pub fn logo() -> Html {
    let (is_dark, _) = use_context::<(bool, Callback<()>)>().expect("Theme context not found");
    let logo_src = if !is_dark {
        "/assets/iggy-dark.png"
    } else {
        "/assets/iggy-light.png"
    };

    html! {
        <div class="logo">
            <img src={logo_src} alt="Apache Iggy Logo" />
            <h1>{"Apache Iggy Benchmarks"}</h1>
        </div>
    }
}
