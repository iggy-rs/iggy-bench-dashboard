use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BenchmarkResultProps {
    pub kind: String,
    pub pretty_name: Option<String>,
    pub selected: bool,
    pub on_select: Callback<String>,
}

#[function_component(BenchmarkResult)]
pub fn benchmark_result(props: &BenchmarkResultProps) -> Html {
    let onclick = {
        let name = props.kind.clone();
        let on_select = props.on_select.clone();
        Callback::from(move |_| {
            on_select.emit(name.clone());
        })
    };

    let display_name = props.pretty_name.as_ref().unwrap_or(&props.kind);

    html! {
        <div class={if props.selected { "benchmark-result selected" } else { "benchmark-result" }}
             onclick={onclick}>
            {display_name.clone()}
        </div>
    }
}
