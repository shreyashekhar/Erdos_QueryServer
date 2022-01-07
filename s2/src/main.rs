// #![deny(warnings)]

use serde::{Deserialize, Serialize};
use std::fs;
use tokio::sync::broadcast::Sender;
use warp::ws::Message;
use warp::Filter;

mod ws;
use ws::{client_connection, forward_broadcast, parse_request, MyRequest};

mod web_server;

use tokio::sync::broadcast;

pub fn test_ws_request(msg: Message) -> Message {
    let response: String;

    match parse_request::<MyRequest>(&msg) {
        Ok(req) => {
            response = req.request_type;
        }
        Err(_e) => {
            response = "Request is not formatted correctly".to_string();
        }
    }

    Message::text(response)
}

pub fn handle_graph_request(_msg: Message) -> Message {
    Message::text(fs::read_to_string("../test.txt").expect("null"))
}

#[derive(Deserialize, Debug)]
pub struct StreamRequest {
    pub stream_id: String,
}

#[derive(Serialize, Debug)]
pub struct StreamResponse {
    pub watermark: i32,
    pub messages_sent: i32,
}

pub fn handle_stream_request(msg: Message) -> Message {
    let response: String;

    match parse_request::<StreamRequest>(&msg) {
        Ok(_) => {
            let pre = StreamResponse {
                watermark: 123,
                messages_sent: 100,
            };

            response = serde_json::to_string(&pre).unwrap();
        }
        Err(_e) => {
            response = "Request is not formatted correctly".to_string();
        }
    }

    Message::text(response)
}

#[derive(Deserialize, Debug)]
pub struct OperatorRequest {
    pub operator_id: String,
}

#[derive(Serialize, Debug)]
pub struct OperatorResponse {
    pub statistics: i32,
}

pub fn handle_operator_request(msg: Message) -> Message {
    let response: String;

    match parse_request::<OperatorRequest>(&msg) {
        Ok(_) => {
            let pre = OperatorResponse { statistics: 32 };
            response = serde_json::to_string(&pre).unwrap();
        }
        Err(_e) => {
            response = "Request is not formatted correctly".to_string();
        }
    }

    Message::text(response)
}

async fn test_background_channel(tx: &Sender<Result<Message, warp::Error>>) {
    loop {
        tx.send(Ok(Message::text("Hello, world!")));
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let echo = warp::path("echo").and(warp::ws()).map(|ws: warp::ws::Ws| {
        ws.on_upgrade(move |socket| client_connection(socket, test_ws_request))
    });

    let graph = warp::path("graph").and(warp::ws()).map(|ws: warp::ws::Ws| {
        ws.on_upgrade(move |socket| client_connection(socket, handle_graph_request))
    });

    let stream = warp::path("stream")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| client_connection(socket, handle_stream_request))
        });

    let operator = warp::path("operator")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| client_connection(socket, handle_operator_request))
        });

    let routes = echo.or(graph).or(stream).or(operator);

    // let (tx, _) = broadcast::channel(16);

    // tokio::spawn(test_background_channel(&tx));

    // routes.or(warp::path("forward")
    //     // The `ws()` filter will prepare the Websocket handshake.
    //     .and(warp::ws())
    //     .map(|ws: warp::ws::Ws| {
    //         let recv = tx.subscribe();
    //         ws.on_upgrade(move |socket| forward_broadcast(socket, recv));
    //     }));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
