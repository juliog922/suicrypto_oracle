/// Test if the server can start and listen on a dynamic port (0).
/// This ensures that the server binds to an available port.
#[tokio::test]
async fn test_server_can_start() {
    use suicrypto_oracle::domain::websocket_server::WebSocketServer;
    use tokio::net::TcpListener;
    
    // Use a dynamic port (0 will automatically select an available port)
    let address = "127.0.0.1:0"; // Dynamic port
    let server = WebSocketServer::new(address).expect("Error creating server");

    // Start the server in a separate task
    let server_task = tokio::spawn(async move {
        if let Err(e) = server.run().await {
            panic!("The server failed with error: {:?}", e);
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let listener = TcpListener::bind(address)
        .await
        .expect("Error binding to port");
    let local_addr = listener.local_addr().expect("Error getting local address");
    println!("Server running on {}", local_addr);

    // Ensure the listener is successfully bound
    assert!(listener.local_addr().is_ok());

    server_task.abort();
}
