#[macro_use]
extern crate lazy_static;

use futures_util::{StreamExt};
use warp::Filter;
use tokio::sync::{mpsc};
use warp::ws::{Message, WebSocket};
use serde::{Serialize, Deserialize};
use tokio_stream::wrappers::{UnboundedReceiverStream};
use std::collections::HashMap;
use std::sync::Mutex;


#[derive(Serialize, Deserialize, Debug)]
pub struct MyRequest {
    pub request_type: String,
}

pub fn test_handler(_msg: Message) -> Message  {
    // let req: MyRequest = serde_json::from_str(msg.to_str().unwrap()).unwrap();
    // Message::text(req.type_)

    Message::text("test".to_string())
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, fn(Message) -> Message>> = 
        Mutex::new(
            HashMap::from([
                ("test".to_string(), test_handler as fn(Message) -> Message),
            ])
        );
}

pub fn parse_request<T: for<'a> Deserialize<'a>>(msg: &Message) -> Result<T, bool> {
    let unparsed: Result<T, serde_json::Error> = serde_json::from_str(msg.to_str().unwrap());
    let response: Result<T, bool> = match unparsed {
        Ok(_) => Ok(unparsed.unwrap()),
        Err(_) => Err(true),
    };
    response
}

pub async fn client_connection(ws: WebSocket, handler_map: &Mutex<HashMap<String, fn(Message) -> Message>>) {
    let (tx, mut rx) = ws.split();
    let (send_in, send_out) = mpsc::unbounded_channel();

    // let send_out_stream = UnboundedReceiverStream::new(send_out);
    
    tokio::task::spawn(send_out.forward(tx));

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

        let response: Message;

        match parse_request::<MyRequest>(&msg) {
            Ok(req) => {
                match &handler_map.lock().unwrap().get(&req.request_type) {
                    Some(handler) => {
                        response = handler(msg);
                    }
                    None => {
                        response = Message::text("Not a valid request type".to_string());
                    }
                }
            },
            Err(_e) => {
                response = Message::text("Missing request type".to_string());
            }
        }

        send_in
            .send(Ok(response))
            .map_err(|err| println!("{:?}", err))
            .ok();
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
            let mapP = & HASHMAP;
            ws.on_upgrade(move |socket| client_connection(socket, mapP))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}