use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::chat::ResponseFormat;

/// Request to create an assistant.
///
/// # Example
///
/// ```rust,ignore
/// use portkey::model::CreateAssistantRequest;
///
/// let request = CreateAssistantRequest::builder()
///     .model("gpt-4")
///     .name("Math Tutor")
///     .instructions("You are a helpful math tutor.")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CreateAssistantRequest {
    /// ID of the model to use.
    pub model: String,

    /// The name of the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The system instructions that the assistant uses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// A list of tool enabled on the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTool>>,

    /// A list of file IDs attached to this assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    /// Set of key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Specifies the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

/// Modifies an existing assistant.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModifyAssistantRequest {
    /// ID of the model to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The name of the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The system instructions that the assistant uses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// A list of tool enabled on the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTool>>,

    /// A list of file IDs attached to this assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    /// Set of key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Specifies the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

/// An assistant object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Assistant {
    /// The identifier of the assistant.
    pub id: String,

    /// The object type, which is always "assistant".
    pub object: String,

    /// The Unix timestamp (in seconds) for when the assistant was created.
    pub created_at: i64,

    /// The name of the assistant.
    pub name: Option<String>,

    /// The description of the assistant.
    pub description: Option<String>,

    /// ID of the model to use.
    pub model: String,

    /// The system instructions that the assistant uses.
    pub instructions: Option<String>,

    /// A list of tool enabled on the assistant.
    pub tools: Vec<AssistantTool>,

    /// A list of file IDs attached to this assistant.
    pub file_ids: Vec<String>,

    /// Set of key-value pairs that can be attached to an object.
    pub metadata: HashMap<String, String>,

    /// What sampling temperature to use, between 0 and 2.
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature.
    pub top_p: Option<f32>,

    /// Specifies the format that the model must output.
    pub response_format: Option<ResponseFormat>,
}

/// Tool enabled on an assistant.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AssistantTool {
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
    #[serde(rename = "retrieval")]
    Retrieval,
    #[serde(rename = "function")]
    Function { function: FunctionDefinition },
}

/// Definition of a function tool.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// The name of the function to be called.
    pub name: String,

    /// A description of what the function does.
    pub description: String,

    /// The parameters the functions accepts, described as a JSON Schema object.
    pub parameters: Value,
}

/// Response containing a list of assistants.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListAssistantsResponse {
    pub object: String,
    pub data: Vec<Assistant>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

/// Response from deleting an assistant.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAssistantResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

/// An assistant file object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssistantFile {
    /// The identifier of the assistant file.
    pub id: String,

    /// The object type, which is always "assistant.file".
    pub object: String,

    /// The Unix timestamp (in seconds) for when the assistant file was created.
    pub created_at: i64,

    /// The assistant ID that the file is attached to.
    pub assistant_id: String,
}

/// Request to create an assistant file.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAssistantFileRequest {
    /// A File ID that the assistant should use.
    pub file_id: String,
}

/// Response containing a list of assistant files.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListAssistantFilesResponse {
    pub object: String,
    pub data: Vec<AssistantFile>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

/// Response from deleting an assistant file.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAssistantFileResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}
