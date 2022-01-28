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
use ws::{client_connection};

pub fn handler_one(_msg: Message) -> Message {
    Message::text("hello from handler 1!".to_string())
}

pub fn handler_two(_msg: Message) -> Message {
    Message::text("what's up from handler 2!".to_string())
}

/// Creates channel and sends messages accross it at a fixed rate.
/// Drops messages if the channel fills up.
fn create_background_stream(stream_no: u8, rate: f32) -> Sender<Message> {
    let (tx, _) = broadcast::channel(16);
    let tx_copy = tx.clone();

    // Send 2 messages / second from another green thread.
    tokio::task::spawn(async move {
        let sleep_dur = Duration::from_secs_f32(1.0 / rate);

        loop {
            tx_copy.send(Message::text(stream_no.to_string())).ok();
            tokio::time::delay_for(sleep_dur).await;
        }
    });

    tx
}

#[tokio::main]
async fn main() {
    // request handlers
    let req_handlers = Arc::new(Mutex::new(HashMap::from([
        ("one".to_string(), handler_one as fn(Message) -> Message),
        ("two".to_string(), handler_two as fn(Message) -> Message),
    ])));

    // data streams
    let bg1 = create_background_stream(1, 2.0);
    let bg2 = create_background_stream(2, 2.0);
    
    let data_streams: Arc<Mutex<Vec<Sender<Message>>>> = Arc::new(Mutex::new(Vec::new()));
    let mut data_streams_g = data_streams.lock().unwrap();
    data_streams_g.push(bg1);
    data_streams_g.push(bg2);
    std::mem::drop(data_streams_g);


    // create the endpoint
    let routes = warp::path("dashboard")
        .and(warp::ws())
        .map( move |ws: warp::ws::Ws| {
            // Make copies of request handlers and data streams
            let req_handlers_copy = req_handlers.clone();
            let data_streams_copy = data_streams.clone();

            ws.on_upgrade(move |socket| client_connection(socket, req_handlers_copy, data_streams_copy))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
