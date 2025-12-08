use std::future::Future;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::model::{ChatCompletionRequest, ChatCompletionResponse};
use crate::{PortkeyClient, Result};

/// Trait for chat completion operations.
///
/// Provides methods for creating chat completions using the Portkey API.
/// This trait is implemented on the [`PortkeyClient`](crate::client::PortkeyClient).
pub trait ChatService {
    /// Creates a chat completion.
    ///
    /// Sends a chat completion request to the configured LLM provider through Portkey's gateway.
    /// The request follows the OpenAI-compatible chat completions format and can be routed to
    /// any supported provider based on your Portkey configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - The chat completion request with model, messages, and optional parameters
    ///
    /// # Returns
    ///
    /// Returns the chat completion response with the model's generated message(s).
    ///
    /// # Authentication Options
    ///
    /// The client must be configured with one of the following authentication methods:
    ///
    /// 1. **Virtual Key**: API key + Virtual key (managed provider credentials)
    /// 2. **Provider Auth**: API key + Provider name + Authorization header
    /// 3. **Config**: API key + Config ID (for complex routing/fallback configurations)
    /// 4. **Custom Host**: API key + Provider name + Authorization + Custom host URL
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{AuthMethod, PortkeyClient, PortkeyConfig, Result};
    /// # use portkey_sdk::model::{ChatCompletionRequest, ChatCompletionRequestMessage, ChatCompletionUserMessageContent};
    /// # use portkey_sdk::service::ChatService;
    /// # async fn example() -> Result<()> {
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-portkey-api-key")
    ///     .with_auth_method(AuthMethod::VirtualKey {
    ///         virtual_key: "your-virtual-key".to_string(),
    ///     })
    ///     .build()?;
    /// let client = PortkeyClient::new(config)?;
    ///
    /// let request = ChatCompletionRequest {
    ///     model: "gpt-4o".to_string(),
    ///     messages: vec![
    ///         ChatCompletionRequestMessage::System {
    ///             content: "You are a helpful assistant.".to_string(),
    ///             name: None,
    ///         },
    ///         ChatCompletionRequestMessage::User {
    ///             content: ChatCompletionUserMessageContent::Text("Hello!".to_string()),
    ///             name: None,
    ///         },
    ///     ],
    ///     temperature: Some(0.7),
    ///     max_tokens: Some(100),
    ///     frequency_penalty: None,
    ///     logit_bias: None,
    ///     logprobs: None,
    ///     top_logprobs: None,
    ///     n: None,
    ///     presence_penalty: None,
    ///     response_format: None,
    ///     seed: None,
    ///     stop: None,
    ///     stream: None,
    ///     stream_options: None,
    ///     thinking: None,
    ///     top_p: None,
    ///     tools: None,
    ///     tool_choice: None,
    ///     parallel_tool_calls: None,
    ///     user: None,
    /// };
    ///
    /// let response = client.create_chat_completion(request).await?;
    /// println!("Response: {:?}", response.choices[0].message.content);
    /// # Ok(())
    /// # }
    /// ```
    fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> impl Future<Output = Result<ChatCompletionResponse>>;
}

impl ChatService for PortkeyClient {
    async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            model = %request.model,
            messages_count = request.messages.len(),
            "Creating chat completion"
        );

        let response = self
            .post("/chat/completions")?
            .json(&request)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let chat_response: ChatCompletionResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            id = %chat_response.id,
            choices_count = chat_response.choices.len(),
            "Chat completion created successfully"
        );

        Ok(chat_response)
    }
}
