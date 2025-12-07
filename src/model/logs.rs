use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Export status enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportStatus {
    Draft,
    InProgress,
    Success,
    Failed,
    Stopped,
}

/// Requested data fields for log exports
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogExportField {
    Id,
    TraceId,
    CreatedAt,
    Request,
    Response,
    IsSuccess,
    AiOrg,
    AiModel,
    ReqUnits,
    ResUnits,
    TotalUnits,
    RequestUrl,
    Cost,
    CostCurrency,
    ResponseTime,
    ResponseStatusCode,
    Mode,
    Config,
    PromptSlug,
    Metadata,
}

/// Filters for log generation queries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenerationsFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_generation_min: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_generation_max: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_units_min: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_units_max: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_token_min: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_token_max: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_token_min: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_token_max: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_org_model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_feedback_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_feedback_max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_keys: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_slug: Option<String>,

    /// Maximum number of items per page (max: 50000, default: 50000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,

    /// Current page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
}

/// Request to create a log export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLogExportRequest {
    /// Workspace ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,

    /// Filters to apply to the export
    pub filters: GenerationsFilter,

    /// Fields to include in the export
    pub requested_data: Vec<LogExportField>,

    /// Description of the export
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response from creating or updating a log export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLogExportResponse {
    /// Export ID
    pub id: String,

    /// Total number of items in the export
    pub total: i32,

    /// Object type (always "export")
    pub object: String,
}

/// Log export item details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogExport {
    /// Export ID
    pub id: String,

    /// Organization ID
    pub organisation_id: String,

    /// Filters applied to this export
    pub filters: GenerationsFilter,

    /// Requested data fields
    pub requested_data: Vec<LogExportField>,

    /// Export status
    pub status: ExportStatus,

    /// Export description
    pub description: String,

    /// Creation timestamp
    pub created_at: String,

    /// Last update timestamp
    pub last_updated_at: String,

    /// User who created the export
    pub created_by: String,

    /// Workspace ID
    pub workspace_id: String,

    /// Object type (always "export")
    pub object: String,
}

/// Response from export task operations (start/cancel)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTaskResponse {
    /// Status message
    pub message: String,

    /// Object type (always "export")
    pub object: String,
}

/// Response from download request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadLogExportResponse {
    /// Pre-signed URL for downloading the export
    pub signed_url: String,
}

/// HTTP request data for a log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRequest {
    /// Request URL
    pub url: String,

    /// HTTP method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// Request headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Request body
    pub body: serde_json::Value,
}

/// HTTP response data for a log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogResponse {
    /// HTTP status code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,

    /// Response headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Response body
    pub body: serde_json::Value,

    /// Response time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<i32>,
}

/// Metadata for a log entry
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogMetadata {
    /// Trace ID for distributed tracing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    /// Span ID for distributed tracing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,

    /// Span name for distributed tracing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_name: Option<String>,

    /// Additional metadata fields
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// A custom log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomLog {
    /// Request data
    pub request: LogRequest,

    /// Response data
    pub response: LogResponse,

    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LogMetadata>,
}

/// Request to insert one or more log entries
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InsertLogRequest {
    /// Single log entry
    Single(Box<CustomLog>),
    /// Multiple log entries
    Multiple(Vec<CustomLog>),
}

/// Response from inserting log entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertLogResponse {
    /// Status message
    pub status: String,

    /// Log IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_ids: Option<Vec<String>>,
}

/// Request to update a log export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLogExportRequest {
    /// Workspace ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,

    /// Filters for the export
    pub filters: GenerationsFilter,

    /// Fields to include in the export
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_data: Option<Vec<LogExportField>>,
}

/// Response from updating a log export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLogExportResponse {
    /// Export ID
    pub id: String,

    /// Total number of items
    pub total: i32,

    /// Object type (always "export")
    pub object: String,
}

// ============================================================================
// List Log Exports Models
// ============================================================================

/// Parameters for listing log exports
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListLogExportsParams {
    /// Workspace ID to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

/// A log export item in a list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogExportListItem {
    /// Export ID
    pub id: String,

    /// Organization ID
    pub organisation_id: String,

    /// Export filters
    pub filters: GenerationsFilter,

    /// Requested data fields
    pub requested_data: Vec<LogExportField>,

    /// Export status
    pub status: ExportStatus,

    /// Export description
    pub description: String,

    /// Creation timestamp
    pub created_at: String,

    /// Last update timestamp
    pub last_updated_at: String,

    /// User who created the export
    pub created_by: String,

    /// Workspace ID
    pub workspace_id: String,

    /// Object type (always "export")
    pub object: String,
}

/// Response from listing log exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLogExportsResponse {
    /// Object type (always "list")
    pub object: String,

    /// Total number of exports
    pub total: i32,

    /// Array of export items
    pub data: Vec<LogExportListItem>,
}
