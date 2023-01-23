use yew_router::prelude::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/room/:id")]
    Room { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
