//! Portkey API client configuration and initialization.
//!
//! This module provides the core client types for interacting with the Portkey API:
//!
//! - [`PortkeyConfig`] - Configuration builder for API settings
//! - [`PortkeyBuilder`] - Builder pattern for creating configurations
//! - [`PortkeyClient`] - Main client for making API requests

mod config;
mod portkey;

pub use config::PortkeyConfig;
pub use portkey::PortkeyClient;

/// Configuration builder types for Portkey clients.
///
/// This module contains the builder pattern types used to construct
/// [`PortkeyConfig`](super::PortkeyConfig) instances with custom settings.
///
/// # Examples
///
/// ```no_run
/// use portkey_sdk::builder::PortkeyBuilder;
///
/// let config = PortkeyBuilder::default()
///     .with_api_key("your-api-key")
///     .build()
///     .unwrap();
/// ```
pub mod builder {
    pub use super::config::{AuthMethod, PortkeyBuilder, PortkeyBuilderError};
}
