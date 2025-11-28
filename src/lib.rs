#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub mod client;

mod error;

#[doc(hidden)]
pub mod prelude;

// Re-exports
pub use client::{PortkeyBuilder, PortkeyBuilderError, PortkeyClient, PortkeyConfig};
pub use error::{Error, Result};
