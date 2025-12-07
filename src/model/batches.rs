use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to create a batch.
///
/// # Example
///
/// ```rust,ignore
/// use portkey::model::CreateBatchRequest;
///
/// let request = CreateBatchRequest::builder()
///     .input_file_id("file-abc123")
///     .endpoint("/v1/chat/completions")
///     .completion_window("24h")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateBatchRequest {
    /// The ID of an uploaded file that contains requests for the new batch.
    pub input_file_id: String,

    /// The endpoint to be used for all requests in the batch.
    /// Currently supported: /v1/chat/completions, /v1/embeddings, /v1/completions
    pub endpoint: String,

    /// The time frame within which the batch should be processed.
    /// Currently only "24h" is supported.
    pub completion_window: String,

    /// Optional custom metadata for the batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl Default for CreateBatchRequest {
    fn default() -> Self {
        Self {
            input_file_id: String::new(),
            endpoint: String::new(),
            completion_window: "24h".to_string(),
            metadata: None,
        }
    }
}

/// The batch object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Batch {
    /// The batch identifier.
    pub id: String,

    /// The object type, which is always "batch".
    pub object: String,

    /// The OpenAI API endpoint used by the batch.
    pub endpoint: String,

    /// Error information for the batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<BatchErrors>,

    /// The ID of the input file for the batch.
    pub input_file_id: String,

    /// The time frame within which the batch should be processed.
    pub completion_window: String,

    /// The current status of the batch.
    pub status: String,

    /// The ID of the file containing the outputs of successfully executed requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file_id: Option<String>,

    /// The ID of the file containing the outputs of requests with errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_file_id: Option<String>,

    /// The Unix timestamp (in seconds) for when the batch was created.
    pub created_at: i64,

    /// The Unix timestamp (in seconds) for when the batch started processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch started finalizing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalizing_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch started cancelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelling_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch was cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<i64>,

    /// The request counts for different statuses within the batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_counts: Option<BatchRequestCounts>,

    /// Set of key-value pairs that you can attach to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Errors that occurred during batch processing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchErrors {
    /// The object type.
    pub object: String,

    /// List of errors.
    pub data: Vec<BatchError>,
}

/// A single error in a batch.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchError {
    /// An error code identifying the error type.
    pub code: String,

    /// A human-readable message providing more details about the error.
    pub message: String,

    /// The name of the parameter that caused the error, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,

    /// The line number of the input file where the error occurred, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
}

/// Request counts for different statuses within the batch.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchRequestCounts {
    /// Total number of requests in the batch.
    pub total: i64,

    /// Number of requests that have been completed successfully.
    pub completed: i64,

    /// Number of requests that have failed.
    pub failed: i64,
}

/// Response containing a list of batches.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListBatchesResponse {
    pub object: String,
    pub data: Vec<Batch>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}
