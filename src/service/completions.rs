//! Completions API service.
//!
//! Provides access to the legacy completions endpoint.

use std::future::Future;

use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{CompletionResponse, CreateCompletionRequest};

/// Service trait for completions operations.
pub trait CompletionsService {
    /// Create a completion for the provided prompt and parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - The completion request parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyConfig, PortkeyClient, Result};
    /// # use portkey_sdk::service::CompletionsService;
    /// # use portkey_sdk::model::{CreateCompletionRequest, CompletionPrompt};
    /// # async fn example() -> Result<()> {
    /// let config = PortkeyConfig::from_env()?;
    /// let client = PortkeyClient::new(config)?;
    ///
    /// let request = CreateCompletionRequest {
    ///     model: "gpt-3.5-turbo-instruct".to_string(),
    ///     prompt: Some(CompletionPrompt::String("Say this is a test".to_string())),
    ///     max_tokens: Some(100),
    ///     temperature: Some(0.7),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.create_completion(request).await?;
    /// println!("Completion: {}", response.choices[0].text);
    /// # Ok(())
    /// # }
    /// ```
    fn create_completion(
        &self,
        request: CreateCompletionRequest,
    ) -> impl Future<Output = Result<CompletionResponse>>;
}

impl CompletionsService for PortkeyClient {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self, request), fields(model = %request.model))
    )]
    async fn create_completion(
        &self,
        request: CreateCompletionRequest,
    ) -> Result<CompletionResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Creating completion"
        );

        let response = self
            .post("/completions")
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<CompletionResponse>()
            .await?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: crate::TRACING_TARGET_SERVICE,
            id = %response.id,
            choices = response.choices.len(),
            "Completion created successfully"
        );

        Ok(response)
    }
}

// Add Default impl for CreateCompletionRequest
impl Default for CreateCompletionRequest {
    fn default() -> Self {
        Self {
            model: String::new(),
            prompt: None,
            max_tokens: Some(16),
            temperature: Some(1.0),
            top_p: Some(1.0),
            n: Some(1),
            stream: Some(false),
            logprobs: None,
            echo: Some(false),
            stop: None,
            frequency_penalty: Some(0.0),
            presence_penalty: Some(0.0),
            best_of: Some(1),
            logit_bias: None,
            user: None,
            suffix: None,
            seed: None,
        }
    }
}
