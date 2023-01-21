use std::{
    collections::HashMap,
    sync::Arc,
};

use crate::run_code::code_handler;
use futures::{stream::SplitSink, lock::Mutex};
use redis::Client;
use room::{health_handler, room_handler};
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

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1").unwrap();
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

    let active_users: Arc<Mutex<HashMap<String, Arc<Mutex<Vec<SplitSink<WebSocket, Message>>>>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let room_route = warp::path!("room" / String)
        .and(warp::ws())
        .and(warp::any().map(move || client.clone()))
        .and(warp::any().map(move || active_users.clone()))
        .and_then(
            |key: String, ws: warp::ws::Ws, client: Client, active_users| async move {
                room_handler(key, ws, &client, active_users).await
            },
        );

    let health_route = warp::path!("health" / String).and_then(health_handler);

    let code_route = warp::path("code")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(code_handler);

    let routes = room_route
        .or(code_route)
        .or(health_route)
        .recover(handle_rejection)
        .with(&cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
