use futures_util::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::{BroadcastStream, UnboundedReceiverStream};
use warp::ws::{Message, WebSocket};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyRequest {
    pub request_type: String,
}

pub async fn client_connection(ws: WebSocket, handler: fn(Message) -> Message) {
    let (tx, mut rx) = ws.split();
    let (send_in, send_out) = mpsc::unbounded_channel();
    let send_out_stream = UnboundedReceiverStream::new(send_out);

    tokio::task::spawn(send_out_stream.forward(tx).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    while let Some(result) = rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(_e) => {
                eprintln!("error");
                break;
            }
        };

        let msg = handler(msg);
        send_in
            .send(Ok(msg))
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    println!("conn");
}

pub async fn forward_broadcast(
    ws: WebSocket,
    input: broadcast::Receiver<Result<Message, warp::Error>>,
) {
    let (tx, _) = ws.split();
    // let (send_in, send_out) = mpsc::unbounded_channel();

    let input_stream = BroadcastStream::new(input);
    tokio::task::spawn(input_stream.forward(tx).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    // input.for_each(|msg| {
    //     let msg = msg.unwrap();
    //     send_in.send(msg).map_err(|err| println!("{:?}", err)).ok();
    //     Ok(())
    // }).await;

    // let uin = BroadcastStream::new(input);
    // tokio::task::spawn(uin.forward(tx).map(|result| {
    //     if let Err(e) = result {
    //         eprintln!("error sending websocket msg: {}", e);
    //     }
    // }));
}

pub fn parse_request<T: for<'a> Deserialize<'a>>(msg: &Message) -> Result<T, bool> {
    let unparsed: Result<T, serde_json::Error> = serde_json::from_str(msg.to_str().unwrap());
    let response: Result<T, bool> = match unparsed {
        Ok(_) => Ok(unparsed.unwrap()),
        Err(_) => Err(true),
    };
    response
}
