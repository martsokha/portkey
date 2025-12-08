use std::future::Future;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{
    PromptCompletionRequest, PromptCompletionResponse, PromptRenderRequest, PromptRenderResponse,
};

/// Service trait for executing prompt templates.
///
/// This trait provides methods for executing saved prompt templates on Portkey,
/// allowing you to substitute variables and override hyperparameters.
///
/// # Example
///
/// ```rust,no_run
/// use portkey_sdk::{AuthMethod, PortkeyConfig, PromptsService, Result};
/// use portkey_sdk::model::PromptCompletionRequest;
/// use std::collections::HashMap;
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
///     let mut variables = HashMap::new();
///     variables.insert("user_input".to_string(), serde_json::json!("Hello world"));
///
///     let request = PromptCompletionRequest {
///         variables,
///         stream: Some(false),
///         max_tokens: Some(250),
///         presence_penalty: Some(0.2),
///         temperature: None,
///         frequency_penalty: None,
///         top_p: None,
///         stop: None,
///         n: None,
///         logprobs: None,
///         echo: None,
///         best_of: None,
///         logit_bias: None,
///         user: None,
///     };
///
///     let response = client.execute_prompt("your-prompt-id", request).await?;
///     println!("Completion: {:?}", response.body);
///
///     Ok(())
/// }
/// ```
pub trait PromptsService {
    /// Executes a saved prompt template with the given variables and parameters.
    ///
    /// # Arguments
    ///
    /// * `prompt_id` - The unique identifier of the prompt template
    /// * `request` - The completion request with variables and hyperparameters
    ///
    /// # Returns
    ///
    /// Returns a `PromptCompletionResponse` containing the completion result.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be parsed.
    fn execute_prompt(
        &self,
        prompt_id: &str,
        request: PromptCompletionRequest,
    ) -> impl Future<Output = Result<PromptCompletionResponse>>;

    /// Renders a prompt template with variables and hyperparameters without executing it.
    ///
    /// This method substitutes variables in the prompt template and applies hyperparameters,
    /// returning the fully rendered configuration that would be sent to the LLM provider.
    ///
    /// # Arguments
    ///
    /// * `prompt_id` - The unique identifier of the prompt template
    /// * `request` - The render request with variables and optional hyperparameters
    ///
    /// # Returns
    ///
    /// Returns a `PromptRenderResponse` containing the rendered prompt configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be parsed.
    fn render_prompt(
        &self,
        prompt_id: &str,
        request: PromptRenderRequest,
    ) -> impl Future<Output = Result<PromptRenderResponse>>;
}

impl PromptsService for PortkeyClient {
    async fn execute_prompt(
        &self,
        prompt_id: &str,
        request: PromptCompletionRequest,
    ) -> Result<PromptCompletionResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            prompt_id = %prompt_id,
            stream = ?request.stream,
            max_tokens = ?request.max_tokens,
            "Executing prompt template"
        );

        let path = format!("/prompts/{}/completions", prompt_id);
        let response = self.post(&path).json(&request).send().await?;
        let response = response.error_for_status()?;
        let completion_response: PromptCompletionResponse = response.json().await?;

        Ok(completion_response)
    }

    async fn render_prompt(
        &self,
        prompt_id: &str,
        request: PromptRenderRequest,
    ) -> Result<PromptRenderResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            prompt_id = %prompt_id,
            max_tokens = ?request.max_tokens,
            "Rendering prompt template"
        );

        let path = format!("/prompts/{}/render", prompt_id);
        let response = self.post(&path).json(&request).send().await?;
        let response = response.error_for_status()?;
        let render_response: PromptRenderResponse = response.json().await?;

        Ok(render_response)
    }
}
