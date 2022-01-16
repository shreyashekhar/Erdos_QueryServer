#[macro_use]
extern crate lazy_static;

use warp::Filter;
use warp::ws::{Message};
use std::collections::HashMap;
use std::sync::Mutex;

mod ws;
use ws::{parse_request, client_connection, MyRequest};


pub fn handler_one(_msg: Message) -> Message  {
    Message::text("hello from handler 1!".to_string())
}

pub fn handler_two(_msg: Message) -> Message  {
    Message::text("what's up from handler 2!".to_string())
}

lazy_static! {
    static ref HANDLERS: Mutex<HashMap<String, fn(Message) -> Message>> = 
        Mutex::new(
            HashMap::from([
                ("one".to_string(), handler_one as fn(Message) -> Message),
                ("two".to_string(), handler_two as fn(Message) -> Message),
            ])
        );
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::path("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            let handlers_p = & HANDLERS;
            ws.on_upgrade(move |socket| client_connection(socket, handlers_p))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}