//! Authentication methods for the Portkey API.
//!
//! This module defines the different authentication methods supported by Portkey
//! for routing requests to various LLM providers.

/// Authentication method for Portkey API.
///
/// Portkey supports multiple authentication methods for routing requests
/// to different LLM providers.
#[derive(Debug, Clone)]
pub enum AuthMethod {
    /// Virtual Key authentication - managed provider credentials in Portkey.
    ///
    /// Uses `x-portkey-virtual-key` header. Virtual keys are managed in the
    /// Portkey dashboard and securely store provider API keys.
    ///
    /// # Example
    /// ```no_run
    /// # use portkey_sdk::AuthMethod;
    /// let auth = AuthMethod::VirtualKey {
    ///     virtual_key: "your-virtual-key".to_string(),
    /// };
    /// ```
    VirtualKey {
        /// The virtual key ID from Portkey dashboard
        virtual_key: String,
    },

    /// Provider authentication with direct provider credentials.
    ///
    /// Uses `x-portkey-provider` and `Authorization` headers to directly
    /// authenticate with a provider.
    ///
    /// # Example
    /// ```no_run
    /// # use portkey_sdk::AuthMethod;
    /// let auth = AuthMethod::ProviderAuth {
    ///     provider: "openai".to_string(),
    ///     authorization: "Bearer sk-...".to_string(),
    ///     custom_host: None,
    /// };
    /// ```
    ProviderAuth {
        /// Provider name (e.g., "openai", "anthropic", "google")
        provider: String,
        /// Authorization header value (e.g., "Bearer sk-...")
        authorization: String,
        /// Optional custom host URL for self-hosted or enterprise endpoints
        custom_host: Option<String>,
    },

    /// Config-based authentication using Portkey configs.
    ///
    /// Uses `x-portkey-config` header. Configs define complex routing,
    /// fallback, and load balancing rules in the Portkey dashboard.
    ///
    /// # Example
    /// ```no_run
    /// # use portkey_sdk::AuthMethod;
    /// let auth = AuthMethod::Config {
    ///     config_id: "pc-config-123".to_string(),
    /// };
    /// ```
    Config {
        /// The config ID from Portkey dashboard
        config_id: String,
    },
}

impl AuthMethod {
    /// Creates a virtual key authentication method.
    ///
    /// # Example
    /// ```no_run
    /// # use portkey_sdk::AuthMethod;
    /// let auth = AuthMethod::virtual_key("your-virtual-key");
    /// ```
    pub fn virtual_key(virtual_key: impl Into<String>) -> Self {
        Self::VirtualKey {
            virtual_key: virtual_key.into(),
        }
    }

    /// Creates a provider authentication method.
    ///
    /// # Example
    /// ```no_run
    /// # use portkey_sdk::AuthMethod;
    /// let auth = AuthMethod::provider_auth("openai", "Bearer sk-...");
    /// ```
    pub fn provider_auth(provider: impl Into<String>, authorization: impl Into<String>) -> Self {
        Self::ProviderAuth {
            provider: provider.into(),
            authorization: authorization.into(),
            custom_host: None,
        }
    }

    /// Creates a provider authentication method with a custom host.
    ///
    /// # Example
    /// ```no_run
    /// # use portkey_sdk::AuthMethod;
    /// let auth = AuthMethod::provider_auth_with_host(
    ///     "openai",
    ///     "Bearer sk-...",
    ///     "https://custom.openai.com"
    /// );
    /// ```
    pub fn provider_auth_with_host(
        provider: impl Into<String>,
        authorization: impl Into<String>,
        custom_host: impl Into<String>,
    ) -> Self {
        Self::ProviderAuth {
            provider: provider.into(),
            authorization: authorization.into(),
            custom_host: Some(custom_host.into()),
        }
    }

    /// Creates a config-based authentication method.
    ///
    /// # Example
    /// ```no_run
    /// # use portkey_sdk::AuthMethod;
    /// let auth = AuthMethod::config("pc-config-123");
    /// ```
    pub fn config(config_id: impl Into<String>) -> Self {
        Self::Config {
            config_id: config_id.into(),
        }
    }
}
