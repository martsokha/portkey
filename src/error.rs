//! Error types for the Portkey SDK.

use crate::builder::PortkeyBuilderError;

/// Error type for Portkey API operations.
///
/// This enum represents all possible errors that can occur when using the Portkey SDK,
/// from HTTP transport errors to API-specific failures and configuration issues.
///
/// # Examples
///
/// Handling different error types:
///
/// ```no_run
/// use portkey_sdk::{Error, Result, PortkeyClient};
/// use portkey_sdk::service::ModelsService;
///
/// # async fn example() -> Result<()> {
/// let client: PortkeyClient = PortkeyClient::from_env()?;
///
/// // Example error handling
/// match client.list_models(None).await {
///     Ok(models) => println!("Found {} models", models.data.len()),
///     Err(Error::Http(e)) => println!("Network error: {}", e),
///     Err(Error::Config(e)) => println!("Configuration error: {}", e),
///     Err(e) => println!("Other error: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP transport error from the underlying HTTP client.
    ///
    /// This includes network connectivity issues, DNS resolution failures,
    /// timeout errors, and other transport-layer problems.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    ///
    /// This occurs when the SDK fails to parse API responses or serialize
    /// request payloads to/from JSON.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Configuration error.
    ///
    /// This occurs when configuration parameters are invalid or when using
    /// the configuration builder and validation fails during the build process.
    #[error("Configuration error: {0}")]
    Config(#[from] PortkeyBuilderError),

    /// URL parsing error.
    ///
    /// This occurs when a provided URL string is invalid or cannot be parsed.
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
}

/// Result type for Portkey API operations.
///
/// This is a convenience type alias for `std::result::Result<T, Error>` that is used
/// throughout the Portkey SDK. All SDK methods that can fail return this Result type.
pub type Result<T, E = Error> = std::result::Result<T, E>;
