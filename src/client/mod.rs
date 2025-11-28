//! Portkey API client configuration and initialization.
//!
//! This module provides the core client types for interacting with the Portkey API:
//!
//! - [`PortkeyConfig`] - Configuration builder for API settings
//! - [`PortkeyBuilder`] - Builder pattern for creating configurations
//! - [`PortkeyClient`] - Main client for making API requests

pub mod config;
mod portkey;

pub use config::{AuthMethod, PortkeyBuilder, PortkeyBuilderError, PortkeyConfig};
pub use portkey::PortkeyClient;
