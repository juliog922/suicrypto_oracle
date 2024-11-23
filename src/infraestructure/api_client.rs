use serde_json::Value;
use chrono::{TimeZone, Utc};
use log::debug;

use crate::AppError;

const API_FETCH_PRICE: &'static str = "https://coins.llama.fi/prices/current/sui";

// API Client responsible for fetching token prices.
#[derive(Debug)]
pub struct ApiClient {
    contract_address: String,
}

impl ApiClient {
    /// Creates a new ApiClient instance.
    pub fn new(contract_address: String) -> Self {
        ApiClient { contract_address }
    }

    /// Fetches the token price from an external API.
    pub async fn fetch_price(&self) -> Result<String, AppError> {
        let url = format!("{}:{}", API_FETCH_PRICE, self.contract_address);
        reqwest::get(&url)
            .await
            .map_err(|e| AppError::ApiError(format!("Error calling {}: {}", url, e)))?
            .text()
            .await
            .map_err(|e| AppError::ApiError(format!("Error getting response from {}: {}", url, e)))
    }

    /// Processes the API response and extracts token price data.
    pub fn process_api_response(response: &str) -> Result<String, AppError> {
        let json: Value = serde_json::from_str(response)
            .map_err(|e| AppError::ApiResponseError(format!("JSON parsing error: {}", e)))?;

        let coins = json
            .get("coins")
            .ok_or(AppError::ApiResponseError("Missing 'coins' key in response".to_string()))?;

        let first_coin = coins
            .as_object()
            .and_then(|obj| obj.values().next())
            .ok_or(AppError::ApiResponseError("No coins found".to_string()))?;

        let symbol = first_coin
            .get("symbol")
            .and_then(|s| s.as_str())
            .ok_or(AppError::ApiResponseError("Missing symbol in response".to_string()))?;
        let price = first_coin
            .get("price")
            .and_then(|p| p.as_f64())
            .ok_or(AppError::ApiResponseError("Missing price in response".to_string()))?;
        let timestamp = first_coin
            .get("timestamp")
            .and_then(|t| t.as_i64())
            .ok_or(AppError::ApiResponseError("Missing timestamp in response".to_string()))?;

        let date_time = Utc.timestamp_opt(timestamp, 0)
            .single()
            .ok_or(AppError::ApiResponseError("Invalid timestamp".to_string()))?;

        let processed = serde_json::json!({
            "symbol": symbol,
            "price": price,
            "timestamp": date_time.to_rfc3339()
        });

        debug!("Processed response: {:?}", processed);
        Ok(processed.to_string())
    }
}