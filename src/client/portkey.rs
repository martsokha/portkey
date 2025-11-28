//! Portkey API client implementation.
//!
//! This module contains the main [`PortkeyClient`] struct and its implementation,
//! providing the core HTTP client functionality for interacting with the Portkey API.

use std::fmt;
use std::sync::Arc;

use reqwest::{Client, RequestBuilder};

use super::config::{AuthMethod, PortkeyConfig};
use crate::error::Result;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CLIENT;

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
    inner: Arc<PortkeyClientInner>,
}

/// Inner client state that is shared via Arc for cheap cloning.
#[derive(Debug)]
struct PortkeyClientInner {
    config: PortkeyConfig,
    client: Client,
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
    fn apply_portkey_headers(&self, mut builder: RequestBuilder) -> RequestBuilder {
        // Always add the Portkey API key
        builder = builder.header("x-portkey-api-key", self.inner.config.api_key());

        // Add authentication method headers
        match self.inner.config.auth_method() {
            AuthMethod::VirtualKey { virtual_key } => {
                builder = builder.header("x-portkey-virtual-key", virtual_key);
            }
            AuthMethod::ProviderAuth {
                provider,
                authorization,
                custom_host,
            } => {
                builder = builder.header("x-portkey-provider", provider);
                builder = builder.header("Authorization", authorization);
                if let Some(host) = custom_host {
                    builder = builder.header("x-portkey-custom-host", host);
                }
            }
            AuthMethod::Config { config_id } => {
                builder = builder.header("x-portkey-config", config_id);
            }
        }

        // Add optional headers
        if let Some(trace_id) = self.inner.config.trace_id() {
            builder = builder.header("x-portkey-trace-id", trace_id);
        }

        if let Some(metadata) = self.inner.config.metadata() {
            if let Ok(metadata_json) = serde_json::to_string(metadata) {
                builder = builder.header("x-portkey-metadata", metadata_json);
            }
        }

        if let Some(cache_namespace) = self.inner.config.cache_namespace() {
            builder = builder.header("x-portkey-cache-namespace", cache_namespace);
        }

        if let Some(cache_force_refresh) = self.inner.config.cache_force_refresh() {
            builder = builder.header(
                "x-portkey-cache-force-refresh",
                cache_force_refresh.to_string(),
            );
        }

        builder
    }

    /// Creates a GET request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "GET", path, url))
    )]
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "GET",
            "Creating HTTP GET request"
        );

        let builder = self
            .inner
            .client
            .get(&url)
            .timeout(self.inner.config.timeout());

        self.apply_portkey_headers(builder)
    }

    /// Creates a POST request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "POST", path, url))
    )]
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "POST",
            "Creating HTTP POST request"
        );

        let builder = self
            .inner
            .client
            .post(&url)
            .timeout(self.inner.config.timeout());

        self.apply_portkey_headers(builder)
    }

    /// Creates a PUT request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "PUT", path, url))
    )]
    pub(crate) fn put(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "PUT",
            "Creating HTTP PUT request"
        );

        let builder = self
            .inner
            .client
            .put(&url)
            .timeout(self.inner.config.timeout());

        self.apply_portkey_headers(builder)
    }

    /// Creates a PATCH request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "PATCH", path, url))
    )]
    pub(crate) fn patch(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "PATCH",
            "Creating HTTP PATCH request"
        );

        let builder = self
            .inner
            .client
            .patch(&url)
            .timeout(self.inner.config.timeout());

        self.apply_portkey_headers(builder)
    }

    /// Creates a DELETE request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "DELETE", path, url))
    )]
    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "DELETE",
            "Creating HTTP DELETE request"
        );

        let builder = self
            .inner
            .client
            .delete(&url)
            .timeout(self.inner.config.timeout());

        self.apply_portkey_headers(builder)
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
