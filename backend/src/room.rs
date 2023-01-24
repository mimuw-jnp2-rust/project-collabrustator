use futures::lock::Mutex;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;

use futures::{SinkExt, StreamExt, TryStreamExt};
use redis::Client;
use warp::{ws::Message, Rejection};

use crate::UsersInRoom;
pub async fn current_room_code_handler(key: String, client: &Client) -> Result<String, Rejection> {
    let mut conn = client.get_connection().unwrap();
    let code = redis::cmd("GET")
        .arg(key)
        .query(&mut conn)
        .unwrap_or_else(|_| String::from(""));
    Ok(code)
}
pub async fn room_handler(
    ws: warp::ws::Ws,
    client: &Client,
    active_users: Arc<Mutex<HashMap<String, Arc<Mutex<UsersInRoom>>>>>,
) -> Result<impl warp::Reply, Rejection> {
    let conn_mutex = Mutex::new(client.get_connection().unwrap());

    Ok(ws.on_upgrade(|mut socket| async move {
        let first_response = socket.next().await.unwrap().unwrap();
        let connect_id = first_response.to_str().unwrap_or("");
        let key = connect_id.to_string();
        let (tx, rx) = socket.split();
        info!("new user connected to room {key}");
        {
            let active_users = active_users.clone();
            //let _ = tx.send(Message::text(code)).await;
            let mut users_locked = active_users.lock().await;
            let room_users = users_locked
                .entry(key.clone())
                .or_insert_with(|| Arc::new(Mutex::new(Vec::new())));
            let mut room_users_lock = room_users.lock().await;
            room_users_lock.push(tx);
        }
        info!("added new user connected to room {key}");
        let _ = rx
            .try_for_each(|message| {
                info!("received message {:?} in room {key}", message);
                let active_users = active_users.clone();
                let conn_mutex = &conn_mutex;
                let key = &key;
                async move {
                    let mut conn = conn_mutex.lock().await;
                    if !message.is_text() {
                        info!("Message is not text");
                        return Ok(());
                    }
                    let key = &key;
                    let active_users = &active_users;
                    let res = String::from_utf8(message.as_bytes().to_vec()).unwrap();
                    let json_res: serde_json::Value = serde_json::from_str(&res).unwrap();
                    if let Some(code) = json_res.get("code") {
                        redis::cmd("SET")
                            .arg(&(*key).clone())
                            .arg(code.as_str())
                            .query(&mut *conn)
                            .unwrap_or_else(|_| String::from(""));
                        drop(conn);
                        info!("Received code update");
                    } else if let Some(_starts_running) = json_res.get("start_running") {
                        info!("Room {key} started executing their code");
                    } else if let Some(_starts_running) = json_res.get("execution_response") {
                        info!("Room {key} ended executing their code");
                    }
                    let message = serde_json::to_string(&json_res).unwrap();
                    info!("Preparing to send message");
                    let mut users_locked = active_users.lock().await;
                    let user_sockets = users_locked.get_mut(&(*key).clone()).unwrap();
                    let mut user_sockets_locked = user_sockets.lock().await;
                    let sockets_iter = user_sockets_locked.iter_mut();
                    info!("Sending message {:?} to all users in room {key}", message);
                    for socket in sockets_iter {
                        let _ = socket
                            .send(Message::text(serde_json::to_string(&message).unwrap()))
                            .await;
                    }
                    info!("Message {:?} sent to all users in room {key}", message);
                    Ok(())
                }
            })
            .await;
    }))
}

pub async fn health_handler(key: String) -> Result<impl warp::Reply, Rejection> {
    Ok(key)
}
