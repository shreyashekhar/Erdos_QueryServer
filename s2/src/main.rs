#![deny(warnings)]

use warp::{Filter};
use warp::ws::{Message};
use std::fs;

mod ws;
use ws::{client_connection, valid, MyRequest};

pub fn handle_ws_request(msg: Message) -> Message  {
    let response: String;

    if valid(&msg) {
        let resp: MyRequest = serde_json::from_str(msg.to_str().unwrap()).unwrap();
        response = resp.request_type;
    } else {
        response = "Request is not formatted correctly".to_string();
    }

    Message::text(response)
}

pub fn handle_graph_request(_msg: Message) -> Message  {
    
    Message::text(fs::read_to_string("../test.txt").expect("null"))
}

pub fn handle_stream_request(msg: Message) -> Message  {
    let response: String;

    if valid(&msg) {
        let resp: MyRequest = serde_json::from_str(msg.to_str().unwrap()).unwrap();
        response = resp.request_type;
    } else {
        response = "Request is not formatted correctly".to_string();
    }

    Message::text(response)
}

pub fn handle_operator_request(msg: Message) -> Message  {
    let response: String;

    if valid(&msg) {
        let resp: MyRequest = serde_json::from_str(msg.to_str().unwrap()).unwrap();
        response = resp.request_type;
    } else {
        response = "Request is not formatted correctly".to_string();
    }

    Message::text(response)
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let echo = warp::path("echo")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| client_connection(socket, handle_ws_request))
        });

    let graph = warp::path("graph").and(warp::ws())
    .map(|ws: warp::ws::Ws| {
        ws.on_upgrade(move |socket| client_connection(socket, handle_graph_request))
    });

    let stream = warp::path("graph").and(warp::ws())
    .map(|ws: warp::ws::Ws| {
        ws.on_upgrade(move |socket| client_connection(socket, handle_stream_request))
    });

    let operator = warp::path("graph").and(warp::ws())
    .map(|ws: warp::ws::Ws| {
        ws.on_upgrade(move |socket| client_connection(socket, handle_operator_request))
    });

    let routes = echo.or(graph);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
