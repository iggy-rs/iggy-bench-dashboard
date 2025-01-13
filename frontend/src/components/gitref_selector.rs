use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GitrefSelectorProps {
    pub versions: Vec<String>,
    pub selected_version: String,
    pub on_version_select: Callback<String>,
}

#[function_component(Gitref)]
pub fn version_selector(props: &GitrefSelectorProps) -> Html {
    let onchange = {
        let on_version_select = props.on_version_select.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                let version = select.value();
                on_version_select.emit(version);
            }
        })
    };

    html! {
        <div class="version-select">
            <h3>{"Version"}</h3>
            <select {onchange} value={props.selected_version.clone()}>
                {
                    props.versions.iter().map(|version| {
                        html! {
                            <option
                                value={version.clone()}
                                selected={version == &props.selected_version}
                            >
                                {version}
                            </option>
                        }
                    }).collect::<Html>()
                }
            </select>
        </div>
    }
}
