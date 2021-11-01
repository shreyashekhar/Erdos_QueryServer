#![deny(warnings)]

use futures_util::{FutureExt, StreamExt};
use warp::Filter;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub fn handle_ws_request(msg: Message, send_in: mpsc::UnboundedSender<Result<Message, warp::Error>>) -> Message {
  Message::text("test")
}

pub async fn client_connection(ws: WebSocket) {
    let (tx, mut rx) = ws.split();
    let (send_in, send_out) = mpsc::unbounded_channel();
    
    tokio::task::spawn(send_out.forward(tx).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    while let Some(result) = rx.next().await {
        let msg = match result {
            Ok(msg) => {
		msg	
	    },
            Err(_e) => {
                eprintln!("error");
                break;
            }
        };
	let msg = handle_ws_request(msg, send_in);
        send_in.send(Ok(msg)).map_err(|err| println!("{:?}", err)).ok();
    }

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
