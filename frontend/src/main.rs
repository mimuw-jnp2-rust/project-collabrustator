use yew_router::prelude::*;
use yew::prelude::*;
pub mod routes;
pub mod room;
pub mod message;
pub mod response;
use crate::routes::Route;
use crate::room::Room;
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Room { id } => html! {
            <Room id={id.clone()}/>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}