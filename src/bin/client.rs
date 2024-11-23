use dotenv::dotenv;
use tokio::sync::broadcast;

use suicrypto_oracle::{application::client_manager::ClientManager, config::Config, AppError};

// Main entry point of the program
#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok(); // Load environment variables
    env_logger::init(); // Initialize logging

    // Load the configuration
    let config = Config::load_from_file("tokens.json")?;

    // Access token list from the configuration
    let tokens = config.tokens;

    // Create a channel for broadcast messages
    let (tx, _) = broadcast::channel::<(String, String)>(100);

    let mut client_manager = ClientManager::new();
    client_manager.create_clients(tokens, tx).await?;

    // Run the clients asynchronously
    client_manager.run_clients().await;

    Ok(())
}
