#![deny(warnings)]

use warp::{Filter};
use warp::ws::{Message};

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

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::path("echo")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| client_connection(socket, handle_ws_request))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
