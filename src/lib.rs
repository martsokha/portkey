#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod model;
pub mod service;

mod error;

#[doc(hidden)]
pub mod prelude;

#[cfg(feature = "tracing")]
pub(crate) const TRACING_TARGET_SERVICE: &str = "portkey_sdk::service";

// Re-exports
pub use client::{
    PortkeyBuilder, PortkeyBuilderError, PortkeyClient, PortkeyConfig, config::AuthMethod,
};
pub use error::{Error, Result};
pub use service::{
    AudioService, ChatService, EmbeddingsService, FeedbackService, ImagesService, LogsService,
    ModelsService, PromptsService, ResponsesService,
};
