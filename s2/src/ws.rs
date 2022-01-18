use futures::TryStreamExt;
use futures_util::{StreamExt};
use tokio::sync::{mpsc};
use warp::ws::{Message, WebSocket};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::pin::Pin;


#[derive(Serialize, Deserialize, Debug)]
pub struct MyRequest {
    pub request_type: String,
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

    let mut rxp = Pin::new(&mut rx);
    
    tokio::task::spawn(send_out.forward(tx));

    loop {
        let result = rxp.try_next().await;
        let msg = match result {
            Ok(msg) => {
		    msg.unwrap()
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