pub mod application;
pub mod infraestructure;
pub mod domain;
pub mod config;

use std::fmt;

/// Enum representing various application errors.
#[derive(Debug)]
pub enum AppError {
    /// Error in TCP connection (e.g., binding or accepting a connection)
    TcpError(String),
    
    /// Error while accepting a WebSocket connection
    WebSocketAcceptError(String),
    
    /// Error while sending or receiving WebSocket messages
    WebSocketMessageError(String),
    
    /// Error in the broadcast channel (e.g., sending messages to subscribers)
    BroadcastError(String),
    
    /// Error while calling an external API
    ApiError(String),
    
    /// Error while processing the response from an API
    ApiResponseError(String),
    
    /// Unknown or unexpected error
    UnknownError(String),
    
    /// Error while handling a file (e.g., reading, writing, or opening)
    FileError(String),
    
    /// Error while processing a JSON response (e.g., parsing)
    JsonError(String),
}

impl fmt::Display for AppError {
    /// Formats the `AppError` enum into a human-readable string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::TcpError(msg) => write!(f, "TCP Error: {}", msg),
            AppError::WebSocketAcceptError(msg) => write!(f, "WebSocket Accept Error: {}", msg),
            AppError::WebSocketMessageError(msg) => write!(f, "WebSocket Message Error: {}", msg),
            AppError::BroadcastError(msg) => write!(f, "Broadcast Channel Error: {}", msg),
            AppError::ApiError(msg) => write!(f, "API Error: {}", msg),
            AppError::ApiResponseError(msg) => write!(f, "API Response Processing Error: {}", msg),
            AppError::UnknownError(msg) => write!(f, "Unknown Error: {}", msg),
            AppError::FileError(msg) => write!(f, "File Handling Error: {}", msg),
            AppError::JsonError(msg) => write!(f, "JSON Processing Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
