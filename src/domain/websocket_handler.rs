use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use std::env;
use tokio::sync::broadcast;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::{infraestructure::api_client::ApiClient, AppError};

// WebSocket handler for managing WebSocket connections and messages.
#[derive(Debug)]
pub struct WebSocketHandler {
    token_name: String,
    contract_address: String,
    #[allow(dead_code)]
    tx: broadcast::Sender<(String, String)>,
}

impl WebSocketHandler {
    /// Creates a new WebSocketHandler instance.
    pub fn new(
        token_name: String,
        contract_address: String,
        tx: broadcast::Sender<(String, String)>,
    ) -> Self {
        WebSocketHandler {
            token_name,
            contract_address,
            tx,
        }
    }

    /// Connects to the WebSocket server and handles incoming messages.
    pub async fn connect(&self) -> Result<(), AppError> {
        // Read the SERVER_HOST environment variable
        let server_host = match env::var("SERVER_HOST") {
            Ok(host) => host,
            Err(_) => {
                return Err(AppError::FileError(
                    "SERVER_HOST not found in .env file".to_string(),
                ));
            }
        };
        let server_url = format!("ws://{}", server_host);

        // Attempt to connect to the WebSocket server
        let (ws_stream, _) = connect_async(&server_url).await.map_err(|e| {
            AppError::TcpError(format!("Error connecting to {}: {}", &server_url, e))
        })?;

        let (mut write, mut read) = ws_stream.split();
        info!("Client connected with Token: {}", self.token_name);

        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                // Check if the message is a request for token price
                if text == "REQUEST_TOKEN_PRICE" {
                    match ApiClient::new(self.contract_address.clone())
                        .fetch_price()
                        .await
                    {
                        Ok(api_response) => {
                            match ApiClient::process_api_response(&api_response) {
                                Ok(processed_response) => {
                                    // Send the processed response back to the WebSocket
                                    write
                                        .send(Message::Text(processed_response))
                                        .await
                                        .map_err(|e| {
                                            AppError::WebSocketMessageError(format!(
                                                "Error sending message: {}",
                                                e
                                            ))
                                        })?;
                                    debug!("Message sent successfully");
                                }
                                Err(e) => {
                                    // Handle API response processing error
                                    write
                                        .send(Message::Text(format!(
                                            "Error processing response: {}",
                                            e
                                        )))
                                        .await
                                        .map_err(|e| {
                                            AppError::WebSocketMessageError(format!(
                                                "Error sending error message: {}",
                                                e
                                            ))
                                        })?;
                                    error!("Error processing API response: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            // Handle API call error
                            write
                                .send(Message::Text(format!("Error calling API: {}", e)))
                                .await
                                .map_err(|e| {
                                    AppError::WebSocketMessageError(format!(
                                        "Error sending API error: {}",
                                        e
                                    ))
                                })?;
                            error!("Error calling the API: {}", e);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
