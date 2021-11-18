#![deny(warnings)]

use futures_util::{FutureExt, StreamExt};
use warp::{Filter};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyRequest {
    request_type: String
}

pub fn valid(msg: &Message) -> bool {
    let unparsed: Result<MyRequest, serde_json::Error> = serde_json::from_str(msg.to_str().unwrap());
    let response: bool = match unparsed {
        Ok(_) => {
            true
        },
        Err(_) => {
            false
        }
    };
    response
}

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

pub async fn client_connection(ws: WebSocket, handler: fn(Message) -> Message) {
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

	    let msg = handler(msg);
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
            ws.on_upgrade(move |socket| client_connection(socket, handle_ws_request))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
