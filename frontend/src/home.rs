use std::time::Duration;

use gloo_timers::callback::Interval;
use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_router::prelude::Link;
use yew_router::{prelude::History, scope_ext::RouterScopeExt};

use crate::routes::Route;

pub struct Home {
    room_id: String,
    carousel_text: Vec<String>,
    text_idx: usize,
    interval: Interval,
}

pub enum Msg {
    CreateRoom,
    RoomIdChanged(String),
    NextText,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Home>) -> Self {
        let carousel_text: Vec<String> = vec![String::from("In a room, you can write Rust code with other people simultaneously and run it remotely. This allows for a more efficient and interactive coding experience."),
        String::from("CollabRustAtor allows you to write Rust code, run it, and collaborate with others in real-time. You can either join an existing room by entering the room's ID or create a new room.")];
        let link = ctx.link().clone();
        let interval = Interval::new(Duration::from_secs(10).as_millis() as u32, move || {
            link.send_message(Msg::NextText)
        });
        Self {
            room_id: String::new(),
            carousel_text,
            text_idx: 0,
            interval,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateRoom => {
                let new_id = uuid::Uuid::new_v4().to_string();
                ctx.link()
                    .history()
                    .unwrap()
                    .push(Route::Room { id: new_id });
                true
            }
            Msg::RoomIdChanged(data) => {
                self.room_id = data;
                true
            }
            Msg::NextText => {
                self.text_idx = (self.text_idx + 1) % self.carousel_text.len();
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::NextText);
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="landing-page">
                <div class="landing-page-main">
                    <div class="get-room">
                        <section class="create-room-section">
                            <button class="create-room-button" onclick={ctx.link().callback(|_| Msg::CreateRoom)}>{"Create a new room"}</button>
                        </section>
                        <section class="join-room-section">
                            <h2>{"Join a room"}</h2>
                            <div class="join-room-by-id">
                                <label for="room-id-input">{"Room ID"}</label>
                                <input id="room-id-input" type="text" value={self.room_id.clone()} oninput={ctx.link().callback(|data: InputEvent| Msg::RoomIdChanged(data.target_unchecked_into::<HtmlInputElement>().value()))}/>
                                <Link<Route> to={Route::Room{id : self.room_id.clone()}} >
                                    <button class="join-room-by-id-button">
                                        {"Join"}
                                    </button>
                                </Link <Route>>
                            </div>
                        </section>
                    </div>
                    <section class="how-to-use-section">
                        <img class="how-to-use-image" src="https://images.unsplash.com/photo-1588196749597-9ff075ee6b5b?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8Mnx8b25saW5lJTIwY29sbGFib3JhdGlvbnxlbnwwfHwwfHw%3D&auto=format&fit=crop&w=500&q=60" alt="Collaborative coding illustration"/>
                        <div class="how-to-use-text">
                            <p>{self.carousel_text[self.text_idx].clone()}</p>
                        </div>
                    </section>
                </div>
            </div>
        }
    }
}
