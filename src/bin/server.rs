use dotenv::dotenv;
use std::env;
use suicrypto_oracle::{domain::websocket_server::WebSocketServer, AppError};

/// The entry point of the application.
#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok(); // Load environment variables from a `.env` file if it exists
    env_logger::init(); // Initialize the logger

    // Read the server host from environment variables
    let server_host = env::var("SERVER_HOST").unwrap_or(String::from("127.0.0.1:8080"));

    // Create and run the WebSocket server
    let server = WebSocketServer::new(&server_host)?;
    server.run().await?;
    Ok(())
}
