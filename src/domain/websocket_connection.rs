// websocket_connection.rs
use crate::AppError;
use futures_util::{SinkExt, StreamExt};
use log::info;
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

/// A WebSocket connection handler.
pub struct WebSocketConnection {
    stream: TcpStream,
    receiver: broadcast::Receiver<String>,
}

impl WebSocketConnection {
    /// Creates a new WebSocket connection handler.
    ///
    /// # Arguments
    /// * `stream` - The TCP stream representing the WebSocket connection.
    /// * `receiver` - The broadcast receiver to listen for messages from the server.
    ///
    /// # Returns
    /// * A `WebSocketConnection` instance to handle the connection.
    pub fn new(stream: TcpStream, receiver: broadcast::Receiver<String>) -> Self {
        Self { stream, receiver }
    }

    /// Handles the WebSocket connection by reading and writing messages.
    ///
    /// It listens for incoming messages from the server and forwards them to the client,
    /// and it also reads messages from the client.
    pub async fn run(mut self) -> Result<(), AppError> {
        // Accept the WebSocket connection
        let ws_stream = accept_async(self.stream)
            .await
            .map_err(|e| AppError::WebSocketAcceptError(e.to_string()))?;

        let (mut write, mut read) = ws_stream.split();

        info!("New client connected.");

        // Task for sending messages to the client
        let mut send_task = tokio::spawn(async move {
            while let Ok(msg) = self.receiver.recv().await {
                if write.send(Message::Text(msg)).await.is_err() {
                    return Err(AppError::WebSocketMessageError(
                        "Error sending message to client".to_string(),
                    ));
                }
            }
            Ok::<(), AppError>(())
        });

        // Task for receiving messages from the client
        let mut receive_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    info!("Message received from client: {}", text);
                }
            }
            Ok::<(), AppError>(())
        });

        // Await the completion of both tasks
        tokio::select! {
            result = &mut send_task => result.map_err(|_| AppError::UnknownError("Error in send task".to_string()))??,
            result = &mut receive_task => result.map_err(|_| AppError::UnknownError("Error in receive task".to_string()))??,
        }

        info!("Client disconnected.");
        Ok(())
    }
}
