use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use futures::{TryStreamExt, stream::SplitStream};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};
use warp::ws::{Message, WebSocket};
use warp::{Error};

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

pub async fn add_req_handlers(
    mut rx: SplitStream<WebSocket>,
    funnel: Arc<Mutex<mpsc::UnboundedSender<Result<Message, Error>>>>, 
    request_handlers: Arc<Mutex<HashMap<String, fn(Message) -> Message>>>
) {
    loop {
        // get next request
        let result = rx.try_next().await;

        // unwrap the message
        let msg = match result {
            Ok(msg) => msg.unwrap(),
            Err(_e) => {
                eprintln!("error");
                break;
            }
        };

        // get the response
        let response: Message;
        match parse_request::<MyRequest>(&msg) {
            Ok(req) => match request_handlers.lock().unwrap().get(&req.request_type) {
                Some(handler) => {
                    response = handler(msg);
                }
                None => {
                    response = Message::text("Not a valid request type".to_string());
                }
            },
            Err(_e) => {
                response = Message::text("Missing request type".to_string());
            }
        }

        // send response to the funnel
        let mut funnel_g = funnel.lock().unwrap();
        funnel_g.send(Ok(response))
            .map_err(|err| println!("{:?}", err))
            .ok();
    }
}

pub async fn add_data_stream(    
    funnel: Arc<Mutex<mpsc::UnboundedSender<Result<Message, Error>>>>, 
    data_stream: broadcast::Sender<Message>
) {
    // subscribe to the data stream
    let ds = &mut data_stream.subscribe();

    loop {
        // await a message
        let msg = ds.recv().await.ok();

        // if there is a message send it to the funnel
        if msg.is_some() {
            let funnel_g = funnel.lock().unwrap();
            funnel_g
                .send(Ok(msg.unwrap()))
                .map_err(|err| println!("{:?}", err))
                .ok();
        }
    }
}

pub async fn client_connection(
    ws: WebSocket,
    req_handlers: Arc<Mutex<HashMap<String, fn(Message) -> Message>>>,
    data_streams: Arc<Mutex<Vec<broadcast::Sender<Message>>>>
) {
    // split up the web socket
    let (tx, rx) = ws.split();

    // create an internal mpsc funnel to combine all streams and send to websocket
    let (funnel_in, funnel_out) = mpsc::unbounded_channel();
    let funnel_in = Arc::new(Mutex::new(funnel_in));

    // forward all messages from the funnel to the output websocket
    tokio::task::spawn(funnel_out.forward(tx));

    // connect the request handlers
    let request_handler_funnel = funnel_in.clone();
    tokio::task::spawn(async {
        add_req_handlers(rx, request_handler_funnel, req_handlers).await
    });

    // connect each data stream
    let data_streams_g = data_streams.lock().unwrap();
    for i in 0..data_streams_g.len() {
        let stream_funnel = funnel_in.clone();
        let data_stream = data_streams_g.get(i).unwrap().clone();

        tokio::task::spawn(async move {
            add_data_stream(stream_funnel, data_stream).await
        });
    }
}
