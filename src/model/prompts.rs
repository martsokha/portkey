use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to execute a prompt template with completions.
///
/// This allows you to execute saved prompt templates on Portkey, substituting
/// variables and optionally overriding hyperparameters.
///
/// # Example
///
/// ```rust
/// use portkey_sdk::model::PromptCompletionRequest;
/// use std::collections::HashMap;
///
/// let mut variables = HashMap::new();
/// variables.insert("user_input".to_string(), serde_json::json!("Hello world"));
///
/// let request = PromptCompletionRequest {
///     variables,
///     stream: Some(false),
///     max_tokens: Some(250),
///     temperature: Some(0.7),
///     presence_penalty: Some(0.2),
///     frequency_penalty: None,
///     top_p: None,
///     stop: None,
///     n: None,
///     logprobs: None,
///     echo: None,
///     best_of: None,
///     logit_bias: None,
///     user: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptCompletionRequest {
    /// Variables to substitute in the prompt template
    pub variables: HashMap<String, serde_json::Value>,

    /// Whether to stream the response (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    // Hyperparameters - these are passed at root level, not nested
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    /// Sampling temperature (0-2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Presence penalty (-2 to 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Frequency penalty (-2 to 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Nucleus sampling parameter (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// Number of completions to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,

    /// Include log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// Echo back the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,

    /// Generate best_of completions server-side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i32>,

    /// Logit bias map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,

    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Response from executing a prompt completion.
///
/// The response contains the status, headers, and body. The body can be
/// either a chat completion or text completion response depending on the
/// prompt template configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptCompletionResponse {
    /// Response status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Response headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, serde_json::Value>>,

    /// Response body - can be chat completion or text completion
    pub body: serde_json::Value,
}

/// Request to render a prompt template with variable substitution.
///
/// This endpoint renders a prompt template by substituting variables and
/// applying hyperparameters, returning the fully rendered prompt configuration
/// without executing it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRenderRequest {
    /// Variables to substitute in the prompt template
    pub variables: HashMap<String, serde_json::Value>,

    // Hyperparameters - these are passed at root level, not nested
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    /// Sampling temperature (0-2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Presence penalty (-2 to 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Frequency penalty (-2 to 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Nucleus sampling parameter (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// Number of completions to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,

    /// Include log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// Echo back the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,

    /// Generate best_of completions server-side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i32>,

    /// Logit bias map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,

    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Response from rendering a prompt template.
///
/// Contains the rendered prompt configuration with variables substituted
/// and hyperparameters applied.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRenderResponse {
    /// Whether the render was successful
    pub success: bool,

    /// The rendered prompt data - can be chat completion or text completion request
    pub data: serde_json::Value,
}
