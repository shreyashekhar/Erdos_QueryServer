#[macro_use]
extern crate lazy_static;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::sync::broadcast::{self, Sender};
use warp::{ws::Message, Filter};

mod ws;
use ws::{client_connection, client_stream, parse_request, MyRequest};

pub fn handler_one(_msg: Message) -> Message {
    Message::text("hello from handler 1!".to_string())
}

pub fn handler_two(_msg: Message) -> Message {
    Message::text("what's up from handler 2!".to_string())
}

lazy_static! {
    static ref HANDLERS: Mutex<HashMap<String, fn(Message) -> Message>> =
        Mutex::new(HashMap::from([
            ("one".to_string(), handler_one as fn(Message) -> Message),
            ("two".to_string(), handler_two as fn(Message) -> Message),
        ]));
}

/// Sends messages at a fixed rate.
/// Drops messages if the channel fills up.
async fn send_messages(tx: Sender<Message>, msgs_per_second: f32) {
    let sleep_dur = Duration::from_secs_f32(1.0 / msgs_per_second);

    loop {
        tx.send(Message::text("stream message")).ok();
        tokio::time::delay_for(sleep_dur).await;
    }
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(16);

    // Send 2 messages / second from another green thread.
    tokio::task::spawn(send_messages(tx.clone(), 2.0));

    println!("Started background stream...");

    pretty_env_logger::init();

    let msg_handlers = Arc::new(Mutex::new(HashMap::from([
        ("one".to_string(), handler_one as fn(Message) -> Message),
        ("two".to_string(), handler_two as fn(Message) -> Message),
    ])));

    let echo = warp::path("echo")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            // Creates a new reference to msg_handlers.
            let msg_handlers_ref = msg_handlers.clone();
            ws.on_upgrade(move |socket| client_connection(socket, msg_handlers_ref))
        });

    let stream = warp::path("stream")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx_copy = tx.clone();
            ws.on_upgrade(move |socket| client_stream(socket, tx_copy))
        });

    let routes = echo.or(stream);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
