use yew_router::prelude::Routable;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/room/:id")]
    Room { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
