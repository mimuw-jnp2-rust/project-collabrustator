use yew::prelude::*;
use yew_router::prelude::*;
pub mod home;
pub mod message;
pub mod response;
pub mod room;
pub mod routes;
use crate::home::Home;
use crate::room::Room;
use crate::routes::Route;
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
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
            <header>
                <Link<Route> to={Route::Home}><h1>{"CollabRustAtor"}</h1></Link<Route>>
            </header>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
