use serde::{Deserialize, Serialize};
use std::fmt;

/// Custom error type for the application
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    /// Database/file system errors
    Database(String),
    /// Ollama API errors
    Ollama(String),
    /// Configuration errors
    Configuration(String),
    /// Validation errors
    Validation(String),
    /// Resource not found
    NotFound(String),
    /// Internal server errors
    Internal(String),
    /// Network errors
    Network(String),
    /// Serialization/Deserialization errors
    Serialization(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Ollama(msg) => write!(f, "Ollama error: {}", msg),
            AppError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Conversions from common error types
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Network(err.to_string())
    }
}

// Allow conversion to String for Tauri commands
impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}

/// Result type alias for application operations
pub type AppResult<T> = Result<T, AppError>;
