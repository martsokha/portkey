//! Embeddings example demonstrating various input formats.
//!
//! This example shows how to create embeddings with different input types and formats.
//!
//! # Running this example
//!
//! ```bash
//! export PORTKEY_API_KEY=your-api-key
//! export PORTKEY_VIRTUAL_KEY=your-virtual-key
//! cargo run --example embeddings --features tracing
//! ```
//!
//! # Tracing
//!
//! Enable tracing output:
//! ```bash
//! RUST_LOG=portkey_sdk=debug cargo run --example embeddings --features tracing
//! ```

use portkey_sdk::model::CreateEmbeddingRequest;
use portkey_sdk::{AuthMethod, EmbeddingsService, PortkeyConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Initialize the client
    let client = PortkeyConfig::builder()
        .with_api_key(
            std::env::var("PORTKEY_API_KEY").unwrap_or_else(|_| "your-portkey-api-key".to_string()),
        )
        .with_auth_method(AuthMethod::VirtualKey {
            virtual_key: std::env::var("PORTKEY_VIRTUAL_KEY")
                .unwrap_or_else(|_| "your-virtual-key".to_string()),
        })
        .build_client()?;

    println!("Creating embeddings...\n");

    // Create embedding for a single string
    let request = CreateEmbeddingRequest::new(
        "text-embedding-ada-002",
        "The quick brown fox jumped over the lazy dog.",
    );

    let response = client.create_embedding(request).await?;
    println!("✓ Generated {} embedding(s)", response.data.len());
    println!("  Model: {}", response.model);
    println!("  Dimensions: {}", response.data[0].embedding.len());
    println!("  Tokens used: {}", response.usage.total_tokens);

    Ok(())
}
