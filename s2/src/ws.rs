
use futures_util::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyRequest {
    pub request_type: String
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