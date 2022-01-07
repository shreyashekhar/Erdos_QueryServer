use futures::stream::{SplitSink, SplitStream};
use tokio::sync::mpsc::UnboundedReceiver;
use warp::ws::{Message, WebSocket};

/// Maintains up-to-date status information for ERDOS applications, and sends
/// and streams new information to connected clients.
/// Clients are connected via websockets.
pub struct ErdosWebServer {}

pub struct ErdosMetadataDB {
    // Store all relevant metadata.
}

/// Server has modules, e.g. for streams, operators, etc.
/// Modules provide a view of the state of the system, and send
/// updates on connections.
/// Clients can "subscribe" to updates (e.g. get updates for stream with ID 1234)
/// and then updates get streamed on the socket.
///
///
/// -> ERDOS metadata
/// -> update server state w/ new metadata
/// -> send updated metadata to relevant clients via sockets
///
///
/// 1. Simplest. ERDOS state that stores everything. Every update gets forwarded
///    to all clients.
/// 2. "Sharding"/efficent updates. ERDOS state stores everything. Only relevant
///    updates get forwarded to clients. Clients can subscribe to specific types
///    of updates.

/// Sends new information to connected clients.
pub struct WebSocketHandler<T> {
    /// Receives internal objects which are converted and forwarded to clients.
    from_erdos_channel: UnboundedReceiver<T>,
    /// Converts an internal object to a warp message.
    to_warp_message: Box<dyn Fn(T) -> Message>,
    /// Receive messages from clients.
    client_streams: Vec<SplitStream<WebSocket>>,
    /// Send messages to clients.
    client_sinks: Vec<SplitSink<WebSocket, Message>>,
}

impl<T> WebSocketHandler<T> {
    pub fn new<F: 'static + Fn(T) -> Message>(
        from_erdos_channel: UnboundedReceiver<T>,
        to_warp_message: F,
    ) -> Self {
        Self {
            from_erdos_channel,
            to_warp_message: Box::new(to_warp_message),
            client_streams: vec![],
            client_sinks: vec![],
        }
    }

    pub async fn run(&mut self) {
        loop {
            let metadata = self.from_erdos_channel.recv().await.unwrap();
            let msg = (self.to_warp_message)(metadata);
            // Send the message on the socket.
        }

        loop {
            let metadata = erdos_state.get_metadata();
            if metadata != last_metadata {
                // Send message on socket
            }
        }
    }

    // pub fn add_client
}
