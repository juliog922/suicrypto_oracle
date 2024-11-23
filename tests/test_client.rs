use reqwest::Client;
use tokio;

// Test for the CoinGecko API health check
#[tokio::test]
async fn test_coingecko_healthcheck() {
    // The URL of the CoinGecko API endpoint for the SUI coin
    let url = "https://api.coingecko.com/api/v3/coins/sui";

    // Create a new HTTP client
    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request to CoinGecko API");

    // Check that the response status code is 200 OK (successful)
    assert_eq!(
        response.status().is_success(),
        true,
        "CoinGecko API is down"
    );

    let body = response.text().await.unwrap();

    // Ensure that the response contains the expected data (detail_platforms)
    assert!(
        body.contains("detail_platforms"),
        "Response body doesn't contain expected data"
    );

    // Ensure that the response contains the expected platform address for SUI
    assert!(
        body.contains("0x2::sui::SUI"),
        "Response body doesn't contain expected data"
    );
}

// Test for the CoinLlama API health check
#[tokio::test]
async fn test_coinllama_healthcheck() {
    // The URL of the CoinLlama API endpoint for the SUI coin
    let url = "https://coins.llama.fi/prices/current/sui:0x2::sui::SUI";

    // Create a new HTTP client
    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request to CoinLlama API");

    // Check that the response status code is 200 OK (successful)
    assert_eq!(
        response.status().is_success(),
        true,
        "CoinLlama API is down"
    );

    // Get the response body as text
    let body = response.text().await.unwrap();

    // Ensure that the response contains the expected data for SUI
    assert!(
        body.contains("SUI"),
        "Response body doesn't contain expected data"
    );
}
