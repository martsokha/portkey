use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents a response from the Portkey API.
///
/// A response contains information about API calls made through Portkey,
/// including metadata, status, and associated input/output items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Unique identifier for the response
    pub id: String,

    /// Timestamp when the response was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Timestamp when the response was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    /// The status of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The model used for this response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The provider used for this response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// Additional metadata associated with the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// The trace ID associated with this response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    /// Request body sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,

    /// Response body received from the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,

    /// Total tokens used in the request and response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,

    /// Number of tokens in the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<i64>,

    /// Number of tokens in the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<i64>,

    /// Time taken to complete the request in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i64>,

    /// Cost of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
}

/// Request body for creating a new response.
///
/// # Example
///
/// ```rust
/// use portkey_sdk::model::CreateResponseRequest;
/// use std::collections::HashMap;
///
/// let mut metadata = HashMap::new();
/// metadata.insert("user_id".to_string(), serde_json::json!("user123"));
///
/// let request = CreateResponseRequest {
///     trace_id: Some("trace-123".to_string()),
///     metadata: Some(metadata),
///     model: Some("gpt-4".to_string()),
///     provider: Some("openai".to_string()),
///     status: Some("success".to_string()),
///     request: None,
///     response: None,
///     total_tokens: Some(150),
///     prompt_tokens: Some(100),
///     completion_tokens: Some(50),
///     latency_ms: Some(1500),
///     cost: Some(0.003),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResponseRequest {
    /// The trace ID to associate with this response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    /// Additional metadata to associate with the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// The model used for this response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The provider used for this response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// The status of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Request body sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,

    /// Response body received from the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,

    /// Total tokens used in the request and response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,

    /// Number of tokens in the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<i64>,

    /// Number of tokens in the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<i64>,

    /// Time taken to complete the request in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i64>,

    /// Cost of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
}

/// Represents an input item associated with a response.
///
/// Input items contain the individual messages or prompts that were
/// part of the request to the AI provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputItem {
    /// Unique identifier for the input item
    pub id: String,

    /// The response ID this input item belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,

    /// Timestamp when the input item was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// The role of the message (e.g., "system", "user", "assistant")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// The content of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Additional metadata for the input item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Response containing a list of input items.
///
/// Returned by the list input items endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInputItemsResponse {
    /// List of input items
    pub data: Vec<InputItem>,

    /// Total count of input items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    /// Whether there are more items available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// Parameters for listing input items.
///
/// Used to paginate through input items for a specific response.
///
/// # Example
///
/// ```rust
/// use portkey_sdk::model::ListInputItemsParams;
///
/// let params = ListInputItemsParams {
///     limit: Some(50),
///     offset: Some(0),
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInputItemsParams {
    /// Maximum number of items to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}
