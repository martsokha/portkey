#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod error;

#[doc(hidden)]
pub mod prelude;

// Re-exports
pub use client::{PortkeyBuilder, PortkeyClient, PortkeyConfig};
pub use error::{Error, Result};
