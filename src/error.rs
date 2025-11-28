//! Error types for the Portkey SDK.

use thiserror::Error;

/// A specialized `Result` type for Portkey SDK operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that can occur when using the Portkey SDK.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Configuration builder error.
    #[error("Configuration error: {0}")]
    Config(#[from] crate::client::config::PortkeyBuilderError),

    /// API error response.
    #[error("API error: {message}")]
    Api {
        /// Error message from the API
        message: String,
        /// HTTP status code
        status: Option<u16>,
    },

    /// Generic error with custom message.
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Creates a new API error.
    pub fn api(message: impl Into<String>, status: Option<u16>) -> Self {
        Self::Api {
            message: message.into(),
            status,
        }
    }

    /// Creates a new generic error.
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}
