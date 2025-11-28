//! Prelude module for convenient imports.
//!
//! The prelude re-exports the most commonly used types and traits from the PortKey SDK,
//! allowing you to import everything you need with a single glob import.

pub use crate::client::PortkeyClient;
pub use crate::config::{PortkeyBuilder, PortkeyConfig};
pub use crate::error::{Error, Result};
