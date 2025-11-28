use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{CreateEmbeddingRequest, CreateEmbeddingResponse};
use std::future::Future;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;

/// Service trait for creating embeddings.
///
/// This trait provides methods for generating embeddings from text input
/// using various embedding models through the Portkey gateway.
///
/// # Example
///
/// ```rust,no_run
/// use portkey_sdk::{AuthMethod, PortkeyConfig, EmbeddingsService, Result};
/// use portkey_sdk::model::{CreateEmbeddingRequest, EmbeddingInput, EncodingFormat};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = PortkeyConfig::builder()
///         .with_api_key("your-portkey-api-key")
///         .with_auth_method(AuthMethod::VirtualKey {
///             virtual_key: "your-virtual-key".to_string(),
///         })
///         .build_client()?;
///
///     // Create embeddings for a single string
///     let request = CreateEmbeddingRequest {
///         model: "text-embedding-ada-002".to_string(),
///         input: EmbeddingInput::String(
///             "The quick brown fox jumped over the lazy dog".to_string()
///         ),
///         encoding_format: Some(EncodingFormat::Float),
///         dimensions: None,
///         user: None,
///     };
///
///     let response = client.create_embedding(request).await?;
///     println!("Generated {} embeddings", response.data.len());
///     println!("Model: {}", response.model);
///     println!("Tokens used: {}", response.usage.total_tokens);
///
///     Ok(())
/// }
/// ```
pub trait EmbeddingsService {
    /// Creates an embedding vector representing the input text.
    ///
    /// # Arguments
    ///
    /// * `request` - The embedding request containing the model and input text
    ///
    /// # Returns
    ///
    /// Returns a `CreateEmbeddingResponse` containing the embedding vectors and usage statistics.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use portkey_sdk::{AuthMethod, PortkeyConfig, EmbeddingsService, Result};
    /// use portkey_sdk::model::{CreateEmbeddingRequest, EmbeddingInput};
    ///
    /// # async fn example() -> Result<()> {
    /// # let client = PortkeyConfig::builder()
    /// #     .with_api_key("key")
    /// #     .with_auth_method(AuthMethod::VirtualKey {
    /// #         virtual_key: "vk".to_string(),
    /// #     })
    /// #     .build_client()?;
    /// let request = CreateEmbeddingRequest {
    ///     model: "text-embedding-3-small".to_string(),
    ///     input: EmbeddingInput::StringArray(vec![
    ///         "Hello, world!".to_string(),
    ///         "Embeddings are useful.".to_string(),
    ///     ]),
    ///     encoding_format: None,
    ///     dimensions: Some(512),
    ///     user: Some("user-123".to_string()),
    /// };
    ///
    /// let response = client.create_embedding(request).await?;
    /// for (i, embedding) in response.data.iter().enumerate() {
    ///     println!("Embedding {}: {} dimensions", i, embedding.embedding.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn create_embedding(
        &self,
        request: CreateEmbeddingRequest,
    ) -> impl Future<Output = Result<CreateEmbeddingResponse>>;
}

impl EmbeddingsService for PortkeyClient {
    async fn create_embedding(
        &self,
        request: CreateEmbeddingRequest,
    ) -> Result<CreateEmbeddingResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            model = %request.model,
            encoding_format = ?request.encoding_format,
            dimensions = ?request.dimensions,
            "Creating embedding"
        );

        let response = self.post("/embeddings").json(&request).send().await?;
        let response = response.error_for_status()?;
        let embedding_response: CreateEmbeddingResponse = response.json().await?;

        Ok(embedding_response)
    }
}
