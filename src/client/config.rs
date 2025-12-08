//! Portkey client configuration and builder.
//!
//! This module provides the configuration types and builder pattern for creating
//! and customizing [`PortkeyClient`] instances.

use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use derive_builder::Builder;
use reqwest::Client;

use super::auth::AuthMethod;
use super::portkey::PortkeyClient;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CONFIG;
use crate::error::Result;

/// Configuration for the Portkey API client.
///
/// This struct holds all the necessary configuration parameters for creating and using
/// a Portkey API client, including authentication credentials, API endpoint information,
/// and HTTP client settings.
///
/// # Examples
///
/// Creating a config with virtual key:
/// ```no_run
/// # use portkey_sdk::PortkeyConfig;
/// # use portkey_sdk::AuthMethod;
/// let config = PortkeyConfig::builder()
///     .with_api_key("your-portkey-api-key")
///     .with_auth_method(AuthMethod::VirtualKey {
///         virtual_key: "your-virtual-key".to_string(),
///     })
///     .build()
///     .unwrap();
/// ```
///
/// Creating a config with provider auth:
/// ```no_run
/// # use portkey_sdk::PortkeyConfig;
/// # use portkey_sdk::AuthMethod;
/// let config = PortkeyConfig::builder()
///     .with_api_key("your-portkey-api-key")
///     .with_auth_method(AuthMethod::ProviderAuth {
///         provider: "openai".to_string(),
///         authorization: "Bearer sk-...".to_string(),
///         custom_host: None,
///     })
///     .build()
///     .unwrap();
/// ```
///
/// Creating a config from environment:
/// ```no_run
/// # use portkey_sdk::PortkeyConfig;
/// // Requires PORTKEY_API_KEY and PORTKEY_VIRTUAL_KEY environment variables
/// let config = PortkeyConfig::from_env().unwrap();
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
    /// This is your Portkey API key from the dashboard (x-portkey-api-key header).
    api_key: String,

    /// Authentication method for provider routing.
    ///
    /// Specifies how to authenticate with LLM providers through Portkey.
    auth_method: AuthMethod,

    /// Base URL for the Portkey API.
    ///
    /// Defaults to the official Portkey API endpoint or can be set to a self-hosted gateway.
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

    /// Optional trace ID for request tracking.
    ///
    /// An ID you can pass to refer to one or more requests later on.
    /// If not provided, Portkey generates a trace ID automatically.
    #[builder(default = "None")]
    trace_id: Option<String>,

    /// Optional metadata to attach to requests.
    ///
    /// Arbitrary metadata that will be logged with your requests in Portkey.
    #[builder(default = "None")]
    metadata: Option<HashMap<String, serde_json::Value>>,

    /// Optional cache namespace.
    ///
    /// Partition your Portkey cache store based on custom strings.
    #[builder(default = "None")]
    cache_namespace: Option<String>,

    /// Optional cache force refresh flag.
    ///
    /// Forces a cache refresh by making a new API call and storing the updated value.
    #[builder(default = "None")]
    cache_force_refresh: Option<bool>,
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

    /// Returns the authentication method.
    pub fn auth_method(&self) -> &AuthMethod {
        &self.auth_method
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

    /// Returns the trace ID, if set.
    pub fn trace_id(&self) -> Option<&str> {
        self.trace_id.as_deref()
    }

    /// Returns the metadata, if set.
    pub fn metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }

    /// Returns the cache namespace, if set.
    pub fn cache_namespace(&self) -> Option<&str> {
        self.cache_namespace.as_deref()
    }

    /// Returns the cache force refresh flag, if set.
    pub fn cache_force_refresh(&self) -> Option<bool> {
        self.cache_force_refresh
    }

    /// Creates a configuration from environment variables.
    ///
    /// # Environment Variables
    ///
    /// **Required:**
    /// - `PORTKEY_API_KEY` - Your Portkey API key
    ///
    /// **Authentication (choose one):**
    /// - `PORTKEY_VIRTUAL_KEY` - Virtual key for managed provider credentials
    /// - `PORTKEY_PROVIDER` + `PORTKEY_AUTHORIZATION` - Direct provider auth
    /// - `PORTKEY_CONFIG` - Config ID for complex routing
    ///
    /// **Optional:**
    /// - `PORTKEY_CUSTOM_HOST` - Custom host URL (with provider auth)
    /// - `PORTKEY_BASE_URL` - Base URL for the API
    /// - `PORTKEY_TIMEOUT_SECS` - Request timeout in seconds
    /// - `PORTKEY_TRACE_ID` - Trace ID for request tracking
    /// - `PORTKEY_CACHE_NAMESPACE` - Cache namespace
    /// - `PORTKEY_CACHE_FORCE_REFRESH` - Force cache refresh (true/false)
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Virtual key authentication
    /// export PORTKEY_API_KEY=your-portkey-api-key
    /// export PORTKEY_VIRTUAL_KEY=your-virtual-key
    ///
    /// # Provider authentication
    /// export PORTKEY_API_KEY=your-portkey-api-key
    /// export PORTKEY_PROVIDER=openai
    /// export PORTKEY_AUTHORIZATION="Bearer sk-..."
    ///
    /// # Config-based authentication
    /// export PORTKEY_API_KEY=your-portkey-api-key
    /// export PORTKEY_CONFIG=pc-config-123
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn from_env() -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CONFIG, "Loading configuration from environment");

        let api_key = std::env::var("PORTKEY_API_KEY").map_err(|_| {
            #[cfg(feature = "tracing")]
            tracing::error!(target: TRACING_TARGET_CONFIG, "PORTKEY_API_KEY environment variable not set");

            PortkeyBuilderError::ValidationError(
                "PORTKEY_API_KEY environment variable not set".to_string(),
            )
        })?;

        // Determine authentication method
        let auth_method = if let Ok(virtual_key) = std::env::var("PORTKEY_VIRTUAL_KEY") {
            AuthMethod::VirtualKey { virtual_key }
        } else if let Ok(provider) = std::env::var("PORTKEY_PROVIDER") {
            let authorization = std::env::var("PORTKEY_AUTHORIZATION").map_err(|_| {
                PortkeyBuilderError::ValidationError(
                    "PORTKEY_AUTHORIZATION required when PORTKEY_PROVIDER is set".to_string(),
                )
            })?;
            let custom_host = std::env::var("PORTKEY_CUSTOM_HOST").ok();
            AuthMethod::ProviderAuth {
                provider,
                authorization,
                custom_host,
            }
        } else if let Ok(config_id) = std::env::var("PORTKEY_CONFIG") {
            AuthMethod::Config { config_id }
        } else {
            return Err(PortkeyBuilderError::ValidationError(
                "One of PORTKEY_VIRTUAL_KEY, PORTKEY_PROVIDER, or PORTKEY_CONFIG must be set"
                    .to_string(),
            )
            .into());
        };

        let mut builder = Self::builder()
            .with_api_key(api_key)
            .with_auth_method(auth_method);

        // Optional: custom base URL
        if let Ok(base_url) = std::env::var("PORTKEY_BASE_URL") {
            #[cfg(feature = "tracing")]
            tracing::debug!(target: TRACING_TARGET_CONFIG, base_url = %base_url, "Using custom base URL");

            builder = builder.with_base_url(base_url);
        }

        // Optional: custom timeout
        if let Ok(timeout_str) = std::env::var("PORTKEY_TIMEOUT_SECS") {
            let timeout_secs = timeout_str.parse::<u64>().map_err(|_| {
                #[cfg(feature = "tracing")]
                tracing::error!(target: TRACING_TARGET_CONFIG, timeout_str = %timeout_str, "Invalid PORTKEY_TIMEOUT_SECS value");

                PortkeyBuilderError::ValidationError(format!(
                    "Invalid PORTKEY_TIMEOUT_SECS value: {}",
                    timeout_str
                ))
            })?;

            #[cfg(feature = "tracing")]
            tracing::debug!(target: TRACING_TARGET_CONFIG, timeout_secs, "Using custom timeout");

            builder = builder.with_timeout(Duration::from_secs(timeout_secs));
        }

        // Optional: trace ID
        if let Ok(trace_id) = std::env::var("PORTKEY_TRACE_ID") {
            builder = builder.with_trace_id(trace_id);
        }

        // Optional: cache namespace
        if let Ok(cache_namespace) = std::env::var("PORTKEY_CACHE_NAMESPACE") {
            builder = builder.with_cache_namespace(cache_namespace);
        }

        // Optional: cache force refresh
        if let Ok(cache_force_refresh_str) = std::env::var("PORTKEY_CACHE_FORCE_REFRESH")
            && let Ok(cache_force_refresh) = cache_force_refresh_str.parse::<bool>()
        {
            builder = builder.with_cache_force_refresh(cache_force_refresh);
        }

        let config = builder.build()?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: TRACING_TARGET_CONFIG,
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
        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "test_virtual_key".to_string(),
            })
            .build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.base_url(), "https://api.portkey.ai/v1");
        assert_eq!(config.timeout(), Duration::from_secs(30));

        Ok(())
    }

    #[test]
    fn test_config_builder_with_custom_values() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "test_virtual_key".to_string(),
            })
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
        let result = PortkeyConfig::builder()
            .with_api_key("")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "test".to_string(),
            })
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_zero_timeout() {
        let result = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "test".to_string(),
            })
            .with_timeout(Duration::from_secs(0))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_excessive_timeout() {
        let result = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "test".to_string(),
            })
            .with_timeout(Duration::from_secs(400))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_masked_api_key() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key_12345")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "test".to_string(),
            })
            .build()?;

        assert_eq!(config.masked_api_key(), "test****");

        Ok(())
    }

    #[test]
    fn test_masked_api_key_short() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("abc")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "test".to_string(),
            })
            .build()?;

        assert_eq!(config.masked_api_key(), "****");

        Ok(())
    }

    #[test]
    fn test_auth_method_virtual_key() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "vk-123".to_string(),
            })
            .build()?;

        matches!(
            config.auth_method(),
            AuthMethod::VirtualKey { virtual_key } if virtual_key == "vk-123"
        );

        Ok(())
    }

    #[test]
    fn test_auth_method_provider_auth() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::ProviderAuth {
                provider: "openai".to_string(),
                authorization: "Bearer sk-123".to_string(),
                custom_host: None,
            })
            .build()?;

        matches!(
            config.auth_method(),
            AuthMethod::ProviderAuth { provider, .. } if provider == "openai"
        );

        Ok(())
    }

    #[test]
    fn test_auth_method_config() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::Config {
                config_id: "pc-config-123".to_string(),
            })
            .build()?;

        matches!(
            config.auth_method(),
            AuthMethod::Config { config_id } if config_id == "pc-config-123"
        );

        Ok(())
    }

    #[test]
    fn test_optional_headers() -> Result<()> {
        let mut metadata = HashMap::new();
        metadata.insert("user_id".to_string(), serde_json::json!("12345"));

        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "vk-123".to_string(),
            })
            .with_trace_id("trace-123")
            .with_metadata(metadata.clone())
            .with_cache_namespace("my-cache")
            .with_cache_force_refresh(true)
            .build()?;

        assert_eq!(config.trace_id(), Some("trace-123"));
        assert_eq!(config.cache_namespace(), Some("my-cache"));
        assert_eq!(config.cache_force_refresh(), Some(true));
        assert!(config.metadata().is_some());

        Ok(())
    }
}
