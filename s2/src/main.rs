#![deny(warnings)]

// use serde::{Deserialize, Serialize};
use warp::{Filter};
use warp::ws::{Message};
use std::collections::HashMap;

mod ws;
use ws::{client_connection, MyRequest, parse_request};


pub fn test_ws_request(msg: Message) -> Message  {
    let response: String;

    match parse_request::<MyRequest>(&msg) {
        Ok(req) => {
            response = req.request_type;
        },
        Err(_e) => {
            response = "Request is not formatted correctly".to_string();
        }
    }

    Message::text(response)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let request_map: HashMap<String, fn(Message) -> Message> = 
        HashMap::from([
            ("test".to_string(), test_ws_request as fn(Message) -> Message),
        ]);

    // let map_p = &request_map;

    let routes = warp::path("connect")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| client_connection(socket, &mut request_map))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
