use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to create a thread.
///
/// # Example
///
/// ```rust,ignore
/// use portkey::model::CreateThreadRequest;
///
/// let request = CreateThreadRequest::builder()
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CreateThreadRequest {
    /// A list of messages to start the thread with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ThreadMessage>>,

    /// Set of key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Modifies a thread.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModifyThreadRequest {
    /// Set of key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// A thread object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thread {
    /// The identifier of the thread.
    pub id: String,

    /// The object type, which is always "thread".
    pub object: String,

    /// The Unix timestamp (in seconds) for when the thread was created.
    pub created_at: i64,

    /// Set of key-value pairs that can be attached to an object.
    pub metadata: HashMap<String, String>,
}

/// Response from deleting a thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteThreadResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

/// A message in a thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreadMessage {
    /// The role of the entity that is creating the message.
    pub role: String,

    /// The content of the message.
    pub content: String,

    /// A list of File IDs that the message should use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    /// Set of key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}
