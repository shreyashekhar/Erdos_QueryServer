use futures_util::{StreamExt};
use warp::Filter;
use tokio::sync::{mpsc};
use warp::ws::{Message, WebSocket};
use serde::{Serialize, Deserialize};
use tokio_stream::wrappers::{UnboundedReceiverStream};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    type_: String
}

pub fn handle_ws_request(msg: Message) -> Message  {
    let req: Request = serde_json::from_str(msg.to_str().unwrap()).unwrap();
    Message::text(req.type_)
}

pub async fn client_connection(ws: WebSocket) {
    let (tx, mut rx) = ws.split();
    let (send_in, send_out) = mpsc::unbounded_channel();

    let send_out_stream = UnboundedReceiverStream::new(send_out);
    
    tokio::task::spawn(send_out_stream.forward(tx));

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

	    let msg = handle_ws_request(msg);
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