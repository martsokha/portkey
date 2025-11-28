//! Client module containing the main Portkey client and configuration.

pub mod config;
pub mod portkey;

pub use config::{PortkeyBuilder, PortkeyConfig};
pub use portkey::PortkeyClient;
