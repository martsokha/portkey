//! Files API models.
//!
//! This module contains models for file upload and management.

use serde::{Deserialize, Serialize};

/// Request body for uploading a file.
#[derive(Debug, Clone)]
pub struct UploadFileRequest {
    /// File to upload (bytes).
    pub file: Vec<u8>,

    /// The filename.
    pub filename: String,

    /// The intended purpose of the uploaded file.
    /// Use "assistants" for Assistants and Message files,
    /// "vision" for Assistants image file inputs,
    /// "batch" for Batch API, and "fine-tune" for Fine-tuning.
    pub purpose: String,
}

/// Response from uploading a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileObject {
    /// The file identifier.
    pub id: String,

    /// The object type (always "file").
    pub object: String,

    /// The size of the file in bytes.
    pub bytes: u64,

    /// The Unix timestamp (in seconds) for when the file was created.
    pub created_at: i64,

    /// The name of the file.
    pub filename: String,

    /// The intended purpose of the file.
    pub purpose: String,

    /// Deprecated. The current status of the file (always "processed").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Deprecated. For details on why a fine-tuning training file failed validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

/// Response from listing files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesResponse {
    /// List of file objects.
    pub data: Vec<FileObject>,

    /// The object type (always "list").
    pub object: String,
}

/// Response from deleting a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileResponse {
    /// The ID of the deleted file.
    pub id: String,

    /// The object type (always "file").
    pub object: String,

    /// Whether the file was successfully deleted.
    pub deleted: bool,
}
