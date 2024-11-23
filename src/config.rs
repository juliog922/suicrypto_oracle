use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::AppError;

// Configuration struct to load token information
#[derive(Debug, Deserialize)]
pub struct Config {
    pub tokens: Vec<String>,
}

impl Config {
    /// Loads configuration from a JSON file.
    pub fn load_from_file(file_path: &str) -> Result<Self, AppError> {
        let path = Path::new(file_path);
        let mut file = File::open(path)
            .map_err(|e| AppError::FileError(format!("Error opening file {}: {}", file_path, e)))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| AppError::FileError(format!("Error reading file {}: {}", file_path, e)))?;

        // Deserialize JSON content into the Config structure
        serde_json::from_str::<Config>(&contents)
            .map_err(|e| AppError::JsonError(format!("Error deserializing JSON file: {}", e)))
    }
}
