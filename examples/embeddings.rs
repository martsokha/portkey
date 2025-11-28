use portkey_sdk::model::{CreateEmbeddingRequest, EmbeddingInput, EncodingFormat};
use portkey_sdk::{AuthMethod, EmbeddingsService, PortkeyConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client with your Portkey API key and auth method
    let client = PortkeyConfig::builder()
        .with_api_key(
            std::env::var("PORTKEY_API_KEY").unwrap_or_else(|_| "your-portkey-api-key".to_string()),
        )
        .with_auth_method(AuthMethod::VirtualKey {
            virtual_key: std::env::var("PORTKEY_VIRTUAL_KEY")
                .unwrap_or_else(|_| "your-virtual-key".to_string()),
        })
        .build_client()?;

    println!("=== Single String Embedding ===\n");

    // Example 1: Create embedding for a single string
    let request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("The quick brown fox jumped over the lazy dog".to_string()),
        encoding_format: Some(EncodingFormat::Float),
        dimensions: None,
        user: Some("user-123".to_string()),
    };

    let response = client.create_embedding(request).await?;
    println!("✓ Generated {} embedding(s)", response.data.len());
    println!("  - Model: {}", response.model);
    println!("  - Prompt tokens: {}", response.usage.prompt_tokens);
    println!("  - Total tokens: {}", response.usage.total_tokens);

    if let Some(first_embedding) = response.data.first() {
        println!(
            "  - Embedding dimensions: {}",
            first_embedding.embedding.len()
        );
        println!(
            "  - First 5 values: {:?}",
            &first_embedding.embedding[..5.min(first_embedding.embedding.len())]
        );
    }

    println!("\n=== Multiple String Embeddings ===\n");

    // Example 2: Create embeddings for multiple strings
    let request = CreateEmbeddingRequest {
        model: "text-embedding-3-small".to_string(),
        input: EmbeddingInput::StringArray(vec![
            "Hello, world!".to_string(),
            "Embeddings are useful for semantic search.".to_string(),
            "Portkey makes it easy to work with multiple AI providers.".to_string(),
        ]),
        encoding_format: Some(EncodingFormat::Float),
        dimensions: Some(512), // Reduce dimensions for text-embedding-3 models
        user: Some("user-123".to_string()),
    };

    let response = client.create_embedding(request).await?;
    println!("✓ Generated {} embeddings", response.data.len());
    println!("  - Model: {}", response.model);
    println!("  - Total tokens: {}", response.usage.total_tokens);

    for (i, embedding) in response.data.iter().enumerate() {
        println!(
            "  - Embedding {}: {} dimensions (index: {})",
            i + 1,
            embedding.embedding.len(),
            embedding.index
        );
    }

    println!("\n=== Token Array Embedding ===\n");

    // Example 3: Create embedding from token array
    // Note: These are example token IDs - in practice, you'd get these from a tokenizer
    let request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::TokenArray(vec![1212, 318, 257, 1332, 13]),
        encoding_format: Some(EncodingFormat::Float),
        dimensions: None,
        user: None,
    };

    let response = client.create_embedding(request).await?;
    println!("✓ Generated embedding from token array");
    println!("  - Model: {}", response.model);
    println!("  - Prompt tokens: {}", response.usage.prompt_tokens);

    if let Some(embedding) = response.data.first() {
        println!("  - Embedding dimensions: {}", embedding.embedding.len());
    }

    println!("\n=== Base64 Encoding Format ===\n");

    // Example 4: Get embeddings in base64 format
    let request = CreateEmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("Base64 encoded embeddings".to_string()),
        encoding_format: Some(EncodingFormat::Base64),
        dimensions: None,
        user: None,
    };

    let response = client.create_embedding(request).await?;
    println!("✓ Generated embedding in base64 format");
    println!("  - Model: {}", response.model);
    println!("  - Encoding format: base64");
    println!("  - Total tokens: {}", response.usage.total_tokens);

    Ok(())
}
