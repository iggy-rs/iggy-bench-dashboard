use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="footer-content">
                <a href="https://iggy.rs" target="_blank" rel="noopener noreferrer">
                    {"iggy.rs"}
                </a>
                <span class="separator">{"|"}</span>
                <a href="https://github.com/iggy-rs" target="_blank" rel="noopener noreferrer">
                    {"GitHub"}
                </a>
                <span class="separator">{"|"}</span>
                {"v"}{env!("CARGO_PKG_VERSION")}
                <span class="separator">{"|"}</span>
                {" 2025 Iggy. Built with ❤️ for the message streaming community."}
            </div>
        </footer>
    }
}
