use room::room_handler;
use crate::run_code::code_handler;
use serde::{Deserialize, Serialize};
use warp::{self, hyper::Method, Filter};
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
    let room_route = warp::path!("room")
        .and_then(room_handler);

    let code_route = warp::path("code")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(code_handler);

    let routes = room_route
        .or(code_route)
        .recover(handle_rejection)
        .with(&cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
