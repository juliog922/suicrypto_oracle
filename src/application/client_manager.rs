use tokio::sync::broadcast;
use log::{error, info, warn};
use serde_json::Value;

use crate::{
    AppError, 
    domain::websocket_handler::WebSocketHandler
};

const COINGECKO_API_COINS: &'static str = "https://api.coingecko.com/api/v3/coins";

#[derive(Debug)]
pub struct Client {
    token_name: String,
    contract_address: String,
    tx: broadcast::Sender<(String, String)>,
}

impl Client {
    /// Creates a new client instance.
    pub fn new(token_name: String, contract_address: String, tx: broadcast::Sender<(String, String)>) -> Self {
        Client {
            token_name,
            contract_address,
            tx,
        }
    }

    /// Starts the client's task by establishing a WebSocket connection.
    pub async fn run(&self) {
        let ws_handler = WebSocketHandler::new(self.token_name.clone(), self.contract_address.clone(), self.tx.clone());
        if let Err(e) = ws_handler.connect().await {
            error!("WebSocket connection error: {}", e);
        }
    }
}

// Manages all clients
#[derive(Debug)]
pub struct ClientManager {
    clients: Vec<Client>,
}

impl ClientManager {
    /// Creates a new ClientManager instance.
    pub fn new() -> Self {
        ClientManager {
            clients: Vec::new(),
        }
    }

    /// Creates clients based on the provided token list.
    pub async fn create_clients(
        &mut self,
        tokens: Vec<String>,
        tx: broadcast::Sender<(String, String)>,
    ) -> Result<(), AppError> {
        for token in tokens {
            let url = format!("{}/{}", COINGECKO_API_COINS, token.to_lowercase());
            let response: Value = reqwest::get(&url)
                .await
                .map_err(|e| AppError::ApiError(format!("Error calling {}: {}", url, e)))?
                .json()
                .await
                .map_err(|e| AppError::ApiError(format!("Error parsing JSON response: {}", e)))?;

            // Handle error if token is not found
            if let Some(error_message) = response.get("error").and_then(|e| e.as_str()) {
                if error_message == "coin not found" {
                    warn!("Token not found: {}. API response: {}", token, error_message);
                    continue; // Skip to next token
                }
            }

            // If no error, create the client
            if let Some(contract_address) = response.pointer("/platforms/sui").and_then(|c| c.as_str()) {
                let client = Client::new(token.clone(), contract_address.to_string(), tx.clone());
                self.clients.push(client);
                info!("Client created: {}", token);
            } else {
                warn!("Contract address not found for token: {}", token);
            }
        }
        Ok(())
    }

    /// Runs tasks for all clients asynchronously.
    pub async fn run_clients(self) {
        let mut handlers = vec![];

        for client in self.clients {
            handlers.push(tokio::spawn(async move {
                client.run().await;
            }));
        }

        // Await all client tasks
        for handler in handlers {
            if let Err(e) = handler.await {
                error!("Error in client task: {}", e);
            }
        }
    }
}
