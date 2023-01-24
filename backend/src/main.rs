use std::{collections::HashMap, sync::Arc};

use crate::run_code::code_handler;
use futures::{lock::Mutex, stream::SplitSink};
use redis::Client;
use room::{current_room_code_handler, health_handler, room_handler};
use serde::{Deserialize, Serialize};
use warp::{
    self,
    hyper::Method,
    ws::{Message, WebSocket},
    Filter,
};
pub mod room;
pub mod run_code;
#[derive(Clone, Serialize, Deserialize)]
pub struct Code {
    code: String,
}

async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(warp::reply::json(&format!("{:?}", err)))
}
pub type UsersInRoom = Vec<SplitSink<WebSocket, Message>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Access-Control-Allow-Headers",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Origin",
            "Accept",
            "X-Requested-With",
            "content-type",
            "Host",
            "Referer",
            "Accept",
            "Content-Length",
        ])
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ]);

    let active_users: Arc<Mutex<HashMap<String, Arc<Mutex<UsersInRoom>>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let room_route = warp::path!("ws" / "room")
        .and(warp::ws())
        .and(warp::any().map(|| redis::Client::open("redis://127.0.0.1").unwrap()))
        .and(warp::any().map(move || active_users.clone()))
        .and_then(
            |ws: warp::ws::Ws, client: Client, active_users| async move {
                room_handler(ws, &client, active_users).await
            },
        );

    let room_code_route = warp::path!("room" / String / "code")
        .and(warp::any().map(|| redis::Client::open("redis://127.0.0.1").unwrap()))
        .and_then(|key: String, client: Client| async move {
            current_room_code_handler(key, &client).await
        });

    let health_route = warp::path!("health" / String).and_then(health_handler);

    let code_route = warp::path!("code" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(code_handler);

    let routes = room_route
        .or(room_code_route)
        .or(code_route)
        .or(health_route)
        .recover(handle_rejection)
        .with(&cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
