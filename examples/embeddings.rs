//! Embeddings example.
//!
//! # Usage
//!
//! ```bash
//! export PORTKEY_API_KEY="your-api-key-here"
//! export PORTKEY_VIRTUAL_KEY="your-virtual-key-here"
//! cargo run --example embeddings
//! ```

use portkey_sdk::model::{CreateEmbeddingRequest, EmbeddingInput};
use portkey_sdk::service::EmbeddingsService;
use portkey_sdk::{PortkeyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = PortkeyClient::from_env()?;

    let request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("The quick brown fox jumps over the lazy dog".to_string()),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = client.create_embedding(request).await?;

    println!("Created {} embedding(s)", response.data.len());
    println!("Embedding dimensions: {}", response.data[0].embedding.len());
    println!("Tokens used: {}", response.usage.total_tokens);

    Ok(())
}
