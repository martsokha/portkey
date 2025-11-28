//! Portkey client configuration and builder.
//!
//! This module provides the configuration types and builder pattern for creating
//! and customizing [`PortkeyClient`] instances.

use std::fmt;
use std::time::Duration;

use derive_builder::Builder;
use reqwest::Client;

use super::portkey::PortkeyClient;
use crate::error::Result;

/// Configuration for the Portkey API client.
///
/// This struct holds all the necessary configuration parameters for creating and using
/// a Portkey API client, including authentication credentials, API endpoint information,
/// and HTTP client settings.
///
/// # Examples
///
/// Creating a config with defaults:
/// ```no_run
/// # use portkey_sdk::PortkeyConfig;
/// let config = PortkeyConfig::builder()
///     .with_api_key("your-api-key")
///     .build()
///     .unwrap();
/// ```
///
/// Creating a config from environment:
/// ```no_run
/// # use portkey_sdk::PortkeyConfig;
/// // Requires PORTKEY_API_KEY environment variable
/// let config = PortkeyConfig::from_env().unwrap();
/// ```
///
/// Custom configuration:
/// ```no_run
/// # use portkey_sdk::PortkeyConfig;
/// # use std::time::Duration;
/// let config = PortkeyConfig::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://api.portkey.ai/v1")
///     .with_timeout(Duration::from_secs(60))
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Builder)]
#[builder(
    name = "PortkeyBuilder",
    pattern = "owned",
    setter(into, strip_option, prefix = "with"),
    build_fn(validate = "Self::validate_config")
)]
pub struct PortkeyConfig {
    /// API key for authentication with the Portkey API.
    ///
    /// You can obtain your API key from the Portkey dashboard.
    api_key: String,

    /// Base URL for the Portkey API.
    ///
    /// Defaults to the official Portkey API endpoint.
    #[builder(default = "Self::default_base_url()")]
    base_url: String,

    /// Timeout for HTTP requests.
    ///
    /// Controls how long the client will wait for API responses before timing out.
    #[builder(default = "Self::default_timeout()")]
    timeout: Duration,

    /// Optional custom reqwest client.
    ///
    /// If provided, this client will be used instead of creating a new one.
    /// This allows for custom configuration of the HTTP client (e.g., proxies, custom headers, etc.).
    #[builder(default = "None")]
    client: Option<Client>,
}

impl PortkeyBuilder {
    /// Returns the default base URL for the Portkey API.
    fn default_base_url() -> String {
        "https://api.portkey.ai/v1".to_string()
    }

    /// Returns the default timeout.
    fn default_timeout() -> Duration {
        Duration::from_secs(30)
    }

    /// Validates the configuration before building.
    fn validate_config(&self) -> Result<(), String> {
        // Validate API key is not empty
        if let Some(ref api_key) = self.api_key
            && api_key.trim().is_empty()
        {
            return Err("API key cannot be empty".to_string());
        }

        // Validate timeout is reasonable
        if let Some(timeout) = self.timeout {
            if timeout.is_zero() {
                return Err("Timeout must be greater than 0".to_string());
            }
            if timeout > Duration::from_secs(300) {
                return Err("Timeout cannot exceed 300 seconds (5 minutes)".to_string());
            }
        }

        Ok(())
    }

    /// Creates a Portkey API client directly from the builder.
    ///
    /// This is a convenience method that builds the configuration and
    /// creates a client in one step. This is the recommended way to
    /// create a client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use portkey_sdk::PortkeyConfig;
    /// let client = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build_client()
    ///     .unwrap();
    /// ```
    pub fn build_client(self) -> Result<PortkeyClient> {
        let config = self.build()?;
        PortkeyClient::new(config)
    }
}

impl PortkeyConfig {
    /// Creates a new configuration builder.
    ///
    /// This is the recommended way to construct a `PortkeyConfig`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use portkey_sdk::PortkeyConfig;
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> PortkeyBuilder {
        PortkeyBuilder::default()
    }

    /// Creates a new Portkey API client using this configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use portkey_sdk::PortkeyConfig;
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    ///
    /// let client = config.build_client().unwrap();
    /// ```
    pub fn build_client(self) -> Result<PortkeyClient> {
        PortkeyClient::new(self)
    }

    /// Returns the API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Returns a masked version of the API key for safe display/logging.
    ///
    /// Shows the first 4 characters followed by "****", or just "****"
    /// if the key is shorter than 4 characters.
    pub fn masked_api_key(&self) -> String {
        if self.api_key.len() > 4 {
            format!("{}****", &self.api_key[..4])
        } else {
            "****".to_string()
        }
    }

    /// Returns the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the timeout duration.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Returns a clone of the custom reqwest client, if one was provided.
    pub(crate) fn client(&self) -> Option<Client> {
        self.client.clone()
    }

    /// Creates a configuration from environment variables.
    ///
    /// Reads the API key from the `PORTKEY_API_KEY` environment variable.
    /// Optionally reads `PORTKEY_BASE_URL` and `PORTKEY_TIMEOUT_SECS` if set.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `PORTKEY_API_KEY` environment variable is not set
    /// - Any environment variable contains an invalid value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use portkey_sdk::PortkeyConfig;
    /// // Set environment variable first:
    /// // export PORTKEY_API_KEY=your-api-key
    /// let config = PortkeyConfig::from_env().unwrap();
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn from_env() -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Loading configuration from environment");

        let api_key = std::env::var("PORTKEY_API_KEY").map_err(|_| {
            #[cfg(feature = "tracing")]
            tracing::error!("PORTKEY_API_KEY environment variable not set");

            PortkeyBuilderError::ValidationError(
                "PORTKEY_API_KEY environment variable not set".to_string(),
            )
        })?;

        let mut builder = Self::builder().with_api_key(api_key);

        // Optional: custom base URL
        if let Ok(base_url) = std::env::var("PORTKEY_BASE_URL") {
            #[cfg(feature = "tracing")]
            tracing::debug!(base_url = %base_url, "Using custom base URL");

            builder = builder.with_base_url(base_url);
        }

        // Optional: custom timeout
        if let Ok(timeout_str) = std::env::var("PORTKEY_TIMEOUT_SECS") {
            let timeout_secs = timeout_str.parse::<u64>().map_err(|_| {
                #[cfg(feature = "tracing")]
                tracing::error!(timeout_str = %timeout_str, "Invalid PORTKEY_TIMEOUT_SECS value");

                PortkeyBuilderError::ValidationError(format!(
                    "Invalid PORTKEY_TIMEOUT_SECS value: {}",
                    timeout_str
                ))
            })?;

            #[cfg(feature = "tracing")]
            tracing::debug!(timeout_secs, "Using custom timeout");

            builder = builder.with_timeout(Duration::from_secs(timeout_secs));
        }

        let config = builder.build()?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            base_url = %config.base_url(),
            timeout = ?config.timeout(),
            "Configuration loaded successfully from environment"
        );

        Ok(config)
    }
}

impl fmt::Debug for PortkeyConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PortkeyConfig")
            .field("api_key", &self.masked_api_key())
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() -> Result<()> {
        let config = PortkeyConfig::builder().with_api_key("test_key").build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.base_url(), "https://api.portkey.ai/v1");
        assert_eq!(config.timeout(), Duration::from_secs(30));

        Ok(())
    }

    #[test]
    fn test_config_builder_with_custom_values() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_base_url("https://custom.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.base_url(), "https://custom.api.com");
        assert_eq!(config.timeout(), Duration::from_secs(60));

        Ok(())
    }

    #[test]
    fn test_config_validation_empty_api_key() {
        let result = PortkeyConfig::builder().with_api_key("").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_zero_timeout() {
        let result = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_timeout(Duration::from_secs(0))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_excessive_timeout() {
        let result = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_timeout(Duration::from_secs(400))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_masked_api_key() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key_12345")
            .build()?;

        assert_eq!(config.masked_api_key(), "test****");

        Ok(())
    }

    #[test]
    fn test_masked_api_key_short() -> Result<()> {
        let config = PortkeyConfig::builder().with_api_key("abc").build()?;

        assert_eq!(config.masked_api_key(), "****");

        Ok(())
    }
}
