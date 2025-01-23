use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GitrefSelectorProps {
    pub gitrefs: Vec<String>,
    pub selected_gitref: String,
    pub on_gitref_select: Callback<String>,
}

#[function_component(GitrefSelector)]
pub fn gitref_selector(props: &GitrefSelectorProps) -> Html {
    let onchange = {
        let on_gitref_select = props.on_gitref_select.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                let gitref = select.value();
                on_gitref_select.emit(gitref);
            }
        })
    };

    html! {
        <div class="gitref-select">
            <h3>{"Version"}</h3>
            <select {onchange} value={props.selected_gitref.clone()}>
                {
                    props.gitrefs.iter().map(|gitref| {
                        html! {
                            <option
                                value={gitref.clone()}
                                selected={gitref == &props.selected_gitref}
                            >
                                {gitref}
                            </option>
                        }
                    }).collect::<Html>()
                }
            </select>
        </div>
    }
}
