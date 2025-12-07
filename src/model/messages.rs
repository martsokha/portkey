use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to create a message.
///
/// # Example
///
/// ```rust,ignore
/// use portkey::model::CreateMessageRequest;
///
/// let request = CreateMessageRequest::builder()
///     .role("user")
///     .content("Hello, how are you?")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateMessageRequest {
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

impl Default for CreateMessageRequest {
    fn default() -> Self {
        Self {
            role: "user".to_string(),
            content: String::new(),
            file_ids: None,
            metadata: None,
        }
    }
}

/// Modifies a message.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModifyMessageRequest {
    /// Set of key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// A message object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// The identifier of the message.
    pub id: String,

    /// The object type, which is always "thread.message".
    pub object: String,

    /// The Unix timestamp (in seconds) for when the message was created.
    pub created_at: i64,

    /// The thread ID that this message belongs to.
    pub thread_id: String,

    /// The role of the entity that created the message.
    pub role: String,

    /// The content of the message.
    pub content: Vec<MessageContent>,

    /// If applicable, the ID of the assistant that authored this message.
    pub assistant_id: Option<String>,

    /// If applicable, the ID of the run associated with the authoring of this message.
    pub run_id: Option<String>,

    /// A list of file IDs that the assistant should use.
    pub file_ids: Vec<String>,

    /// Set of key-value pairs that can be attached to an object.
    pub metadata: HashMap<String, String>,
}

/// Content of a message.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "text")]
    Text { text: TextContent },
    #[serde(rename = "image_file")]
    ImageFile { image_file: ImageFileContent },
}

/// Text content in a message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextContent {
    /// The data that makes up the text.
    pub value: String,

    /// Annotations for the text.
    pub annotations: Vec<Annotation>,
}

/// Image file content in a message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageFileContent {
    /// The File ID of the image.
    pub file_id: String,
}

/// An annotation in text content.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Annotation {
    #[serde(rename = "file_citation")]
    FileCitation {
        text: String,
        file_citation: FileCitation,
        start_index: usize,
        end_index: usize,
    },
    #[serde(rename = "file_path")]
    FilePath {
        text: String,
        file_path: FilePathAnnotation,
        start_index: usize,
        end_index: usize,
    },
}

/// A citation within the message that points to a specific quote from a specific File.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileCitation {
    /// The ID of the specific File the citation is from.
    pub file_id: String,

    /// The specific quote in the file.
    pub quote: String,
}

/// A URL for the file that's generated when the assistant used the code_interpreter tool.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilePathAnnotation {
    /// The ID of the file that was generated.
    pub file_id: String,
}

/// Response containing a list of messages.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListMessagesResponse {
    pub object: String,
    pub data: Vec<Message>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

/// A message file object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageFile {
    /// The identifier of the message file.
    pub id: String,

    /// The object type, which is always "thread.message.file".
    pub object: String,

    /// The Unix timestamp (in seconds) for when the message file was created.
    pub created_at: i64,

    /// The ID of the message that the File is attached to.
    pub message_id: String,
}

/// Response containing a list of message files.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListMessageFilesResponse {
    pub object: String,
    pub data: Vec<MessageFile>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}
