use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/single")]
    Single,
    #[not_found]
    #[at("/404")]
    NotFound,
}
