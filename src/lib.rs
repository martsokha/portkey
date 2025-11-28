#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod client;
mod config;
mod error;

#[doc(hidden)]
pub mod prelude;

// Re-exports
pub use client::PortkeyClient;
pub use config::{PortkeyBuilder, PortkeyConfig};
pub use error::{Error, Result};
