#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

// Compile-time check: ensure at least one TLS backend is enabled
#[cfg(not(any(feature = "rustls-tls", feature = "native-tls")))]
compile_error!(
    "At least one TLS backend must be enabled. \
     Enable either the 'rustls-tls' (recommended) or 'native-tls' feature. \
     Example: cargo build --features rustls-tls"
);

pub mod client;
mod error;
pub mod model;
#[doc(hidden)]
pub mod prelude;
pub mod service;

pub use client::{AuthMethod, PortkeyBuilder, PortkeyBuilderError, PortkeyClient, PortkeyConfig};
pub use error::{Error, Result};
pub use service::{
    AudioService, ChatService, EmbeddingsService, FeedbackService, ImagesService, LogsService,
    ModelsService, PromptsService, ResponsesService,
};

/// Tracing target for client-level operations (HTTP requests, client creation).
#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub const TRACING_TARGET_CLIENT: &str = "portkey_sdk::client";

/// Tracing target for configuration operations (config creation, validation).
#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub const TRACING_TARGET_CONFIG: &str = "portkey_sdk::config";

/// Tracing target for service-level operations (API calls, business logic).
#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub const TRACING_TARGET_SERVICE: &str = "portkey_sdk::service";
