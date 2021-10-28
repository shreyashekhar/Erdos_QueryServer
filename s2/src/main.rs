#![deny(warnings)]

use futures_util::{FutureExt, StreamExt};
// use futures_sink::{Sink};
use warp::Filter;
// use std::sync::mpsc;
use tokio::sync::mpsc;
use warp::ws::{WebSocket};

pub async fn client_connection(ws: WebSocket) {
    let (tx, mut rx) = ws.split();
    // let (read_in, read_out) = mpsc::unbounded_channel();
    let (send_in, send_out) = mpsc::unbounded_channel();

    tokio::task::spawn(send_out.forward(tx).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    // rx.forward(read_in);

    while let Some(result) = rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(_e) => {
                eprintln!("error");
                break;
            }
        };
        send_in.send(Ok(msg)).map_err(|err| println!("{:?}", err)).ok();
    }

    // tokio::task::spawn(read_out.for_each(|item| {
    //     send_in.send(item).map_err(|err| println!("{:?}", err)).ok()
    // }));

    println!("conn");
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::path("echo")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| client_connection(socket))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
