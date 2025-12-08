//! Portkey API client implementation.
//!
//! This module contains the main [`PortkeyClient`] struct and its implementation,
//! providing the core HTTP client functionality for interacting with the Portkey API.

use std::fmt;
use std::sync::Arc;

use reqwest::multipart::Form;
use reqwest::{Client, Method, RequestBuilder, Response};

use super::auth::AuthMethod;
use super::config::PortkeyConfig;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CLIENT;
use crate::error::Result;

/// Main Portkey API client for interacting with all Portkey services.
///
/// The `PortkeyClient` provides access to all Portkey API endpoints through specialized
/// service interfaces. It handles authentication, request/response serialization,
/// and provides a consistent async interface for all operations.
///
/// # Features
///
/// - **Thread-safe**: Safe to use across multiple threads
/// - **Cheap to clone**: Uses `Arc` internally for efficient cloning
/// - **Automatic authentication**: Handles API key authentication automatically
///
/// # Examples
///
/// ## Basic usage with environment configuration
///
/// ```no_run
/// use portkey_sdk::{PortkeyClient, Result};
///
/// # async fn example() -> Result<()> {
/// let client = PortkeyClient::from_env()?;
/// # Ok(())
/// # }
/// ```
///
/// ## Custom configuration with builder pattern
///
/// ```no_run
/// use portkey_sdk::{PortkeyConfig, PortkeyClient, Result};
/// use std::time::Duration;
///
/// # async fn example() -> Result<()> {
/// let client = PortkeyConfig::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://api.portkey.ai/v1")
///     .with_timeout(Duration::from_secs(30))
///     .build_client()?;
/// # Ok(())
/// # }
/// ```
///
/// ## Multi-threaded usage
///
/// The client is cheap to clone (uses `Arc` internally):
///
/// ```no_run
/// use portkey_sdk::{PortkeyClient, Result};
/// use tokio::task;
///
/// # async fn example() -> Result<()> {
/// let client = PortkeyClient::from_env()?;
///
/// let handles: Vec<_> = (0..3).map(|i| {
///     let client = client.clone();
///     task::spawn(async move {
///         // Use client here
///         Ok::<(), portkey_sdk::Error>(())
///     })
/// }).collect();
///
/// for handle in handles {
///     handle.await.unwrap()?;
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct PortkeyClient {
    pub(crate) inner: Arc<PortkeyClientInner>,
}

/// Inner client state that is shared via Arc for cheap cloning.
#[derive(Debug)]
pub(crate) struct PortkeyClientInner {
    pub(crate) config: PortkeyConfig,
    pub(crate) client: Client,
}

impl PortkeyClient {
    /// Creates a new Portkey API client.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(config), fields(api_key = %config.masked_api_key())))]
    pub fn new(config: PortkeyConfig) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, "Creating Portkey client");

        let client = if let Some(custom_client) = config.client() {
            custom_client
        } else {
            Client::builder().timeout(config.timeout()).build()?
        };

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: TRACING_TARGET_CLIENT,
            base_url = %config.base_url(),
            timeout = ?config.timeout(),
            api_key = %config.masked_api_key(),
            custom_client = config.client().is_some(),
            "Portkey client created successfully"
        );

        let inner = Arc::new(PortkeyClientInner { config, client });
        Ok(Self { inner })
    }

    /// Creates a new configuration builder for constructing a Portkey client.
    ///
    /// This is a convenience method that returns a `PortkeyBuilder` for building
    /// a custom client configuration.
    ///
    /// # Example
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::builder::AuthMethod;
    /// # use std::time::Duration;
    /// # async fn example() -> Result<()> {
    /// let client = PortkeyClient::builder()
    ///     .with_api_key("your-api-key")
    ///     .with_auth_method(AuthMethod::VirtualKey {
    ///         virtual_key: "your-virtual-key".to_string(),
    ///     })
    ///     .with_timeout(Duration::from_secs(60))
    ///     .build_client()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> crate::builder::PortkeyBuilder {
        PortkeyConfig::builder()
    }

    /// Creates a new Portkey API client from environment variables.
    ///
    /// This is a convenience method that creates a PortkeyConfig from environment
    /// variables and then creates a client from that config.
    ///
    /// # Environment Variables
    ///
    /// - `PORTKEY_API_KEY` - Your Portkey API key (required)
    /// - `PORTKEY_BASE_URL` - Base URL for the API (optional, defaults to <https://api.portkey.ai/v1>)
    /// - `PORTKEY_TIMEOUT_SECS` - Request timeout in seconds (optional, defaults to 30)
    ///
    /// # Example
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # async fn example() -> Result<()> {
    /// let client = PortkeyClient::from_env()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn from_env() -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, "Creating Portkey client from environment");

        let config = PortkeyConfig::from_env()?;
        Self::new(config)
    }

    /// Applies Portkey-specific headers to a request builder.
    ///
    /// This method adds all required and optional Portkey headers to the request.
    /// If metadata serialization fails, it logs a warning and continues without the metadata header.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self, builder), fields(auth_method))
    )]
    fn apply_portkey_headers(&self, mut builder: RequestBuilder) -> RequestBuilder {
        // Always add the Portkey API key
        builder = builder.header("x-portkey-api-key", self.inner.config.api_key());

        // Add authentication method headers
        match self.inner.config.auth_method() {
            AuthMethod::VirtualKey { virtual_key } => {
                #[cfg(feature = "tracing")]
                tracing::trace!(target: TRACING_TARGET_CLIENT, "Using virtual key authentication");

                builder = builder.header("x-portkey-virtual-key", virtual_key);
            }
            AuthMethod::ProviderAuth {
                provider,
                authorization,
                custom_host,
            } => {
                #[cfg(feature = "tracing")]
                tracing::trace!(target: TRACING_TARGET_CLIENT, provider = %provider, "Using provider authentication");

                builder = builder.header("x-portkey-provider", provider);
                builder = builder.header("Authorization", authorization);
                if let Some(host) = custom_host {
                    builder = builder.header("x-portkey-custom-host", host);
                }
            }
            AuthMethod::Config { config_id } => {
                #[cfg(feature = "tracing")]
                tracing::trace!(target: TRACING_TARGET_CLIENT, config_id = %config_id, "Using config-based authentication");

                builder = builder.header("x-portkey-config", config_id);
            }
        }

        // Add optional headers
        if let Some(trace_id) = self.inner.config.trace_id() {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: TRACING_TARGET_CLIENT, trace_id = %trace_id, "Adding trace ID");

            builder = builder.header("x-portkey-trace-id", trace_id);
        }

        if let Some(metadata) = self.inner.config.metadata() {
            match serde_json::to_string(metadata) {
                Ok(metadata_json) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(target: TRACING_TARGET_CLIENT, "Adding metadata header");

                    builder = builder.header("x-portkey-metadata", metadata_json);
                }
                Err(_e) => {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(target: TRACING_TARGET_CLIENT, error = %_e, "Failed to serialize metadata, skipping header");
                }
            }
        }

        if let Some(cache_namespace) = self.inner.config.cache_namespace() {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: TRACING_TARGET_CLIENT, cache_namespace = %cache_namespace, "Adding cache namespace");

            builder = builder.header("x-portkey-cache-namespace", cache_namespace);
        }

        if let Some(cache_force_refresh) = self.inner.config.cache_force_refresh() {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: TRACING_TARGET_CLIENT, cache_force_refresh, "Adding cache force refresh");

            builder = builder.header(
                "x-portkey-cache-force-refresh",
                cache_force_refresh.to_string(),
            );
        }

        builder
    }

    /// Parses the base URL and appends the given path.
    fn parse_url(&self, path: &str) -> Result<url::Url> {
        let mut url = url::Url::parse(self.inner.config.base_url())?;
        url.set_path(&format!("{}{}", url.path().trim_end_matches('/'), path));
        Ok(url)
    }

    /// Builds a URL with the given path and optional query parameters.
    fn build_url(&self, path: &str, params: &[(&str, &str)]) -> Result<url::Url> {
        let mut url = self.parse_url(path)?;

        if !params.is_empty() {
            url.query_pairs_mut().extend_pairs(params);
        }

        Ok(url)
    }

    /// Creates an HTTP request with the specified method.
    fn request(&self, method: Method, url: url::Url) -> RequestBuilder {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: TRACING_TARGET_CLIENT,
            url = %url,
            method = %method,
            "Creating HTTP request"
        );

        let builder = self
            .inner
            .client
            .request(method, url)
            .timeout(self.inner.config.timeout());

        self.apply_portkey_headers(builder)
    }

    /// Sends a GET request and returns the response.
    pub(crate) async fn send(&self, method: Method, path: &str) -> Result<Response> {
        let url = self.parse_url(path)?;
        let response = self.request(method, url).send().await?;
        Ok(response)
    }

    /// Sends a request with JSON body.
    pub(crate) async fn send_json<T: serde::Serialize>(
        &self,
        method: Method,
        path: &str,
        data: &T,
    ) -> Result<Response> {
        let url = self.parse_url(path)?;
        let response = self.request(method, url).json(data).send().await?;
        Ok(response)
    }

    /// Sends a request with query parameters.
    pub(crate) async fn send_with_params(
        &self,
        method: Method,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<Response> {
        let url = self.build_url(path, params)?;
        let response = self.request(method, url).send().await?;
        Ok(response)
    }

    /// Sends a request with multipart form data.
    pub(crate) async fn send_multipart(
        &self,
        method: Method,
        path: &str,
        form: Form,
    ) -> Result<Response> {
        let url = self.parse_url(path)?;
        let response = self.request(method, url).multipart(form).send().await?;
        Ok(response)
    }

    /// Creates a request builder for custom query parameter building.
    /// Use this for complex query scenarios that need conditional parameters.
    pub(crate) fn request_builder(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = self.parse_url(path)?;
        Ok(self.request(method, url))
    }
}

impl fmt::Debug for PortkeyClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PortkeyClient")
            .field("api_key", &self.inner.config.masked_api_key())
            .field("base_url", &self.inner.config.base_url())
            .field("timeout", &self.inner.config.timeout())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn create_test_config() -> PortkeyConfig {
        PortkeyConfig::builder()
            .with_api_key("test_api_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "test_virtual_key".to_string(),
            })
            .build()
            .unwrap()
    }

    #[test]
    fn test_client_creation() -> Result<()> {
        let config = create_test_config();
        let client = PortkeyClient::new(config)?;

        assert_eq!(client.inner.config.api_key(), "test_api_key");
        assert_eq!(client.inner.config.base_url(), "https://api.portkey.ai/v1");

        Ok(())
    }

    #[test]
    fn test_client_creation_with_custom_config() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("custom_key")
            .with_auth_method(AuthMethod::ProviderAuth {
                provider: "openai".to_string(),
                authorization: "Bearer sk-test".to_string(),
                custom_host: None,
            })
            .with_base_url("https://custom.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        let client = PortkeyClient::new(config)?;

        assert_eq!(client.inner.config.api_key(), "custom_key");
        assert_eq!(client.inner.config.base_url(), "https://custom.api.com");
        assert_eq!(client.inner.config.timeout(), Duration::from_secs(60));

        Ok(())
    }

    #[test]
    fn test_client_clone() -> Result<()> {
        let config = create_test_config();
        let client = PortkeyClient::new(config)?;
        let cloned = client.clone();

        assert_eq!(client.inner.config.api_key(), cloned.inner.config.api_key());
        assert_eq!(
            client.inner.config.base_url(),
            cloned.inner.config.base_url()
        );

        Ok(())
    }

    #[test]
    fn test_builder_convenience_method() -> Result<()> {
        let client = PortkeyClient::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "test_vk".to_string(),
            })
            .build_client()?;

        assert_eq!(client.inner.config.api_key(), "test_key");

        Ok(())
    }

    #[test]
    fn test_debug_impl_masks_api_key() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("secret_api_key_12345")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "vk-123".to_string(),
            })
            .build()?;

        let client = PortkeyClient::new(config)?;
        let debug_output = format!("{:?}", client);

        assert!(debug_output.contains("secr****"));
        assert!(!debug_output.contains("secret_api_key_12345"));

        Ok(())
    }

    #[test]
    fn test_auth_method_virtual_key() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "vk-test".to_string(),
            })
            .build()?;

        let client = PortkeyClient::new(config)?;

        matches!(
            client.inner.config.auth_method(),
            AuthMethod::VirtualKey { virtual_key } if virtual_key == "vk-test"
        );

        Ok(())
    }

    #[test]
    fn test_auth_method_provider_auth() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::ProviderAuth {
                provider: "anthropic".to_string(),
                authorization: "Bearer token".to_string(),
                custom_host: Some("https://custom.host".to_string()),
            })
            .build()?;

        let client = PortkeyClient::new(config)?;

        matches!(
            client.inner.config.auth_method(),
            AuthMethod::ProviderAuth { provider, .. } if provider == "anthropic"
        );

        Ok(())
    }

    #[test]
    fn test_auth_method_config() -> Result<()> {
        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::Config {
                config_id: "pc-123".to_string(),
            })
            .build()?;

        let client = PortkeyClient::new(config)?;

        matches!(
            client.inner.config.auth_method(),
            AuthMethod::Config { config_id } if config_id == "pc-123"
        );

        Ok(())
    }

    #[test]
    fn test_optional_headers_config() -> Result<()> {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("key".to_string(), serde_json::json!("value"));

        let config = PortkeyConfig::builder()
            .with_api_key("test_key")
            .with_auth_method(AuthMethod::VirtualKey {
                virtual_key: "vk-test".to_string(),
            })
            .with_trace_id("trace-123")
            .with_metadata(metadata)
            .with_cache_namespace("my-cache")
            .with_cache_force_refresh(true)
            .build()?;

        let client = PortkeyClient::new(config)?;

        assert_eq!(client.inner.config.trace_id(), Some("trace-123"));
        assert_eq!(client.inner.config.cache_namespace(), Some("my-cache"));
        assert_eq!(client.inner.config.cache_force_refresh(), Some(true));
        assert!(client.inner.config.metadata().is_some());

        Ok(())
    }
}
