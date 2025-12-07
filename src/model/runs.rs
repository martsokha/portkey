use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::assistants::{AssistantTool, ResponseFormat};

/// Request to create a run.
///
/// # Example
///
/// ```rust,ignore
/// use portkey::model::CreateRunRequest;
///
/// let request = CreateRunRequest::builder()
///     .assistant_id("asst_abc123")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRunRequest {
    /// The ID of the assistant to use to execute this run.
    pub assistant_id: String,

    /// The ID of the Model to be used to execute this run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Override the default system message of the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Appends additional instructions at the end of the instructions for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_instructions: Option<String>,

    /// Override the tools the assistant can use for this run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTool>>,

    /// Set of key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// The maximum number of prompt tokens that may be used over the course of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_prompt_tokens: Option<i32>,

    /// The maximum number of completion tokens that may be used over the course of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,

    /// Controls for how a thread will be truncated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation_strategy: Option<TruncationStrategy>,

    /// Controls which (if any) tool is called by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Specifies the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

impl Default for CreateRunRequest {
    fn default() -> Self {
        Self {
            assistant_id: String::new(),
            model: None,
            instructions: None,
            additional_instructions: None,
            tools: None,
            metadata: None,
            temperature: None,
            top_p: None,
            max_prompt_tokens: None,
            max_completion_tokens: None,
            truncation_strategy: None,
            tool_choice: None,
            response_format: None,
        }
    }
}

/// Modifies a run.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModifyRunRequest {
    /// Set of key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Request to submit tool outputs to run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitToolOutputsRequest {
    /// A list of tools for which the outputs are being submitted.
    pub tool_outputs: Vec<ToolOutput>,
}

/// Output from a tool.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolOutput {
    /// The ID of the tool call.
    pub tool_call_id: String,

    /// The output of the tool call.
    pub output: String,
}

/// A run object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Run {
    /// The identifier of the run.
    pub id: String,

    /// The object type, which is always "thread.run".
    pub object: String,

    /// The Unix timestamp (in seconds) for when the run was created.
    pub created_at: i64,

    /// The ID of the thread that was executed on as a part of this run.
    pub thread_id: String,

    /// The ID of the assistant used for execution of this run.
    pub assistant_id: String,

    /// The status of the run.
    pub status: String,

    /// Details on the action required to continue the run.
    pub required_action: Option<RequiredAction>,

    /// The last error associated with this run.
    pub last_error: Option<RunError>,

    /// The Unix timestamp (in seconds) for when the run will expire.
    pub expires_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run was started.
    pub started_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run was cancelled.
    pub cancelled_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run failed.
    pub failed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run was completed.
    pub completed_at: Option<i64>,

    /// The model that the assistant used for this run.
    pub model: String,

    /// The instructions that the assistant used for this run.
    pub instructions: String,

    /// The list of tools that the assistant used for this run.
    pub tools: Vec<AssistantTool>,

    /// The list of File IDs the assistant used for this run.
    pub file_ids: Vec<String>,

    /// Set of key-value pairs that can be attached to an object.
    pub metadata: HashMap<String, String>,

    /// Usage statistics related to the run.
    pub usage: Option<RunUsage>,

    /// What sampling temperature was used for this run.
    pub temperature: Option<f32>,

    /// The nucleus sampling value used for this run.
    pub top_p: Option<f32>,

    /// The maximum number of prompt tokens specified for this run.
    pub max_prompt_tokens: Option<i32>,

    /// The maximum number of completion tokens specified for this run.
    pub max_completion_tokens: Option<i32>,

    /// Controls for how a thread will be truncated.
    pub truncation_strategy: Option<TruncationStrategy>,

    /// Controls which (if any) tool is called by the model.
    pub tool_choice: Option<ToolChoice>,

    /// Specifies the format that the model must output.
    pub response_format: Option<ResponseFormat>,
}

/// Details on the action required to continue the run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequiredAction {
    /// The type of action required.
    #[serde(rename = "type")]
    pub action_type: String,

    /// Details on the tool outputs needed for this run to continue.
    pub submit_tool_outputs: SubmitToolOutputs,
}

/// Details on the tool outputs needed for this run to continue.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitToolOutputs {
    /// A list of the relevant tool calls.
    pub tool_calls: Vec<ToolCall>,
}

/// A tool call that the assistant made.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    /// The ID of the tool call.
    pub id: String,

    /// The type of tool call.
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The function definition.
    pub function: FunctionCall,
}

/// A function call.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    /// The name of the function.
    pub name: String,

    /// The arguments to call the function with.
    pub arguments: String,
}

/// Error information for a failed run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunError {
    /// One of server_error, rate_limit_exceeded, or invalid_prompt.
    pub code: String,

    /// A human-readable description of the error.
    pub message: String,
}

/// Usage statistics for a run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunUsage {
    /// Number of completion tokens used.
    pub completion_tokens: i32,

    /// Number of prompt tokens used.
    pub prompt_tokens: i32,

    /// Total number of tokens used.
    pub total_tokens: i32,
}

/// Controls for how a thread will be truncated prior to the run.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TruncationStrategy {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "last_messages")]
    LastMessages { last_messages: i32 },
}

/// Controls which tool is called by the model.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    Auto(String),
    None(String),
    Required(String),
    Function { function: FunctionToolChoice },
}

/// Specifies a tool the model should use.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionToolChoice {
    /// The name of the function to call.
    pub name: String,
}

/// Response containing a list of runs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListRunsResponse {
    pub object: String,
    pub data: Vec<Run>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

/// A run step object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunStep {
    /// The identifier of the run step.
    pub id: String,

    /// The object type, which is always "thread.run.step".
    pub object: String,

    /// The Unix timestamp (in seconds) for when the run step was created.
    pub created_at: i64,

    /// The ID of the assistant associated with the run step.
    pub assistant_id: String,

    /// The ID of the thread that was run.
    pub thread_id: String,

    /// The ID of the run that this run step is a part of.
    pub run_id: String,

    /// The type of run step.
    #[serde(rename = "type")]
    pub step_type: String,

    /// The status of the run step.
    pub status: String,

    /// The details of the run step.
    pub step_details: StepDetails,

    /// The last error associated with this run step.
    pub last_error: Option<RunError>,

    /// The Unix timestamp (in seconds) for when the run step expired.
    pub expired_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run step was cancelled.
    pub cancelled_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run step failed.
    pub failed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run step completed.
    pub completed_at: Option<i64>,

    /// Set of key-value pairs that can be attached to an object.
    pub metadata: HashMap<String, String>,

    /// Usage statistics related to the run step.
    pub usage: Option<RunUsage>,
}

/// Details of a run step.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StepDetails {
    #[serde(rename = "message_creation")]
    MessageCreation { message_creation: MessageCreation },
    #[serde(rename = "tool_calls")]
    ToolCalls { tool_calls: Vec<ToolCall> },
}

/// Details of a message creation step.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageCreation {
    /// The ID of the message that was created by this run step.
    pub message_id: String,
}

/// Response containing a list of run steps.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListRunStepsResponse {
    pub object: String,
    pub data: Vec<RunStep>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}
