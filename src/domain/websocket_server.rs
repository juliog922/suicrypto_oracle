// websocket_server.rs
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use log::{info, warn, error};
use crate::AppError;
use super::websocket_connection::WebSocketConnection;

/// A WebSocket server that listens for incoming WebSocket connections and broadcasts messages to clients.
pub struct WebSocketServer {
    address: String,
    broadcaster: broadcast::Sender<String>,
}

impl WebSocketServer {
    /// Creates a new WebSocket server.
    /// 
    /// # Arguments
    /// * `address` - A string slice containing the address to bind the server.
    ///
    /// # Returns
    /// * `Ok(Self)` if the server was successfully created.
    /// * `Err(AppError)` if an error occurred during initialization.
    pub fn new(address: &str) -> Result<Self, AppError> {
        let (tx, _) = broadcast::channel(16);
        Ok(Self {
            address: address.to_string(),
            broadcaster: tx,
        })
    }

    /// Starts the WebSocket server and listens for incoming connections.
    ///
    /// It handles client connections asynchronously and periodically broadcasts messages to all clients.
    pub async fn run(&self) -> Result<(), AppError> {
        // Bind the server to the specified address
        let listener = TcpListener::bind(&self.address)
            .await
            .map_err(|e| AppError::TcpError(e.to_string()))?;
        
        info!("Server listening on {}", &self.address);

        let broadcaster = self.broadcaster.clone();
        
        // Periodically send a "REQUEST_TOKEN_PRICE" message to all clients
        tokio::spawn(async move {
            loop {
                if broadcaster.send("REQUEST_TOKEN_PRICE".to_string()).is_err() {
                    warn!("{}", AppError::BroadcastError("No clients listening".to_string()));
                }
                sleep(Duration::from_secs(10)).await;
            }
        });

        // Accept incoming connections
        while let Ok((stream, _)) = listener.accept().await {
            let tx = self.broadcaster.clone();
            let rx = tx.subscribe();

            // Spawn a new task to handle the WebSocket connection
            tokio::spawn(async move {
                if let Err(e) = WebSocketConnection::new(stream, rx).run().await {
                    error!("Error in connection: {}", e);
                }
            });
        }

        Ok(())
    }
}
