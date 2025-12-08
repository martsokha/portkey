use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Request to create feedback for a trace.
///
/// # Example
///
/// ```rust
/// use portkey_sdk::model::CreateFeedbackRequest;
/// use std::collections::HashMap;
///
/// let mut metadata = HashMap::new();
/// metadata.insert("user_id".to_string(), serde_json::json!("user123"));
///
/// let request = CreateFeedbackRequest {
///     trace_id: "trace-abc-123".to_string(),
///     value: 5,
///     weight: Some(1.0),
///     metadata: Some(metadata),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFeedbackRequest {
    /// Unique identifier for the request trace
    pub trace_id: String,

    /// Feedback value (integer between -10 and 10)
    pub value: i32,

    /// Weight of the feedback (float between 0 and 1, default: 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,

    /// Additional metadata for the feedback
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Request to update existing feedback.
///
/// # Example
///
/// ```rust
/// use portkey_sdk::model::UpdateFeedbackRequest;
///
/// let request = UpdateFeedbackRequest {
///     value: 8,
///     weight: Some(0.9),
///     metadata: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFeedbackRequest {
    /// Updated feedback value (integer between -10 and 10)
    pub value: i32,

    /// Updated weight of the feedback (float between 0 and 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,

    /// Updated metadata for the feedback
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Response from creating or updating feedback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackResponse {
    /// Status of the operation (success or failure)
    pub status: String,

    /// Confirmation message
    pub message: String,

    /// IDs of feedbacks created/updated
    pub feedback_ids: Vec<String>,
}
