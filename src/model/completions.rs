//! Completions API models.
//!
//! This module contains models for the legacy completions endpoint.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Request body for creating a completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompletionRequest {
    /// ID of the model to use.
    pub model: String,

    /// The prompt(s) to generate completions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<CompletionPrompt>,

    /// Maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Sampling temperature (0-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Nucleus sampling parameter (0-1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Number of completions to generate (1-128).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Whether to stream back partial progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Include log probabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,

    /// Echo back the prompt in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,

    /// Stop sequences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<CompletionStop>,

    /// Frequency penalty (-2 to 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Presence penalty (-2 to 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Generate best_of completions server-side (0-20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,

    /// Modify the likelihood of specified tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Suffix to append after completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// Seed for deterministic sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

/// Prompt can be a string or array of strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionPrompt {
    /// Single string prompt.
    String(String),
    /// Array of string prompts.
    Array(Vec<String>),
}

/// Stop sequences can be a string or array of strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionStop {
    /// Single stop sequence.
    String(String),
    /// Multiple stop sequences (up to 4).
    Array(Vec<String>),
}

/// Response from the completions endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// Unique identifier for the completion.
    pub id: String,

    /// Object type (always "text_completion").
    pub object: String,

    /// Unix timestamp of when the completion was created.
    pub created: i64,

    /// Model used for completion.
    pub model: String,

    /// List of completion choices.
    pub choices: Vec<CompletionChoice>,

    /// Usage statistics for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<CompletionUsage>,

    /// System fingerprint for the backend configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
}

/// A single completion choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionChoice {
    /// Generated completion text.
    pub text: String,

    /// Index of this choice in the list.
    pub index: u32,

    /// Reason for completion finishing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,

    /// Log probabilities for tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<CompletionLogprobs>,
}

/// Log probability information for tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionLogprobs {
    /// List of tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<String>>,

    /// Log probabilities for each token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_logprobs: Option<Vec<f32>>,

    /// Text offset for each token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_offset: Option<Vec<u32>>,

    /// Top log probabilities for each position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<HashMap<String, f32>>>,
}

/// Token usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionUsage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u32,

    /// Number of tokens in the completion.
    pub completion_tokens: u32,

    /// Total tokens used (prompt + completion).
    pub total_tokens: u32,
}
