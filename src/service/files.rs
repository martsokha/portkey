//! Files API service.
//!
//! Provides access to file upload and management endpoints.

use std::future::Future;

use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{DeleteFileResponse, FileObject, ListFilesResponse, UploadFileRequest};

/// Service trait for file operations.
pub trait FilesService {
    /// Upload a file that can be used across various endpoints.
    ///
    /// # Arguments
    ///
    /// * `request` - The file upload request with file bytes, filename, and purpose
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FilesService;
    /// # use portkey_sdk::model::UploadFileRequest;
    /// # async fn example() -> Result<()> {
    /// let client = PortkeyClient::from_env()?;
    ///
    /// let request = UploadFileRequest {
    ///     file: vec![/* file bytes */],
    ///     filename: "training_data.jsonl".to_string(),
    ///     purpose: "fine-tune".to_string(),
    /// };
    ///
    /// let file = client.upload_file(request).await?;
    /// println!("Uploaded file: {}", file.id);
    /// # Ok(())
    /// # }
    /// ```
    fn upload_file(&self, request: UploadFileRequest) -> impl Future<Output = Result<FileObject>>;

    /// Returns a list of files that belong to the user's organization.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FilesService;
    /// # async fn example() -> Result<()> {
    /// let client = PortkeyClient::from_env()?;
    ///
    /// let files = client.list_files().await?;
    /// println!("Found {} files", files.data.len());
    /// # Ok(())
    /// # }
    /// ```
    fn list_files(&self) -> impl Future<Output = Result<ListFilesResponse>>;

    /// Returns information about a specific file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to retrieve
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FilesService;
    /// # async fn example() -> Result<()> {
    /// let client = PortkeyClient::from_env()?;
    ///
    /// let file = client.retrieve_file("file-abc123").await?;
    /// println!("File: {}", file.filename);
    /// # Ok(())
    /// # }
    /// ```
    fn retrieve_file(&self, file_id: &str) -> impl Future<Output = Result<FileObject>>;

    /// Returns the contents of the specified file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to retrieve content from
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FilesService;
    /// # async fn example() -> Result<()> {
    /// let client = PortkeyClient::from_env()?;
    ///
    /// let content = client.retrieve_file_content("file-abc123").await?;
    /// println!("File size: {} bytes", content.len());
    /// # Ok(())
    /// # }
    /// ```
    fn retrieve_file_content(&self, file_id: &str) -> impl Future<Output = Result<Vec<u8>>>;

    /// Delete a file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FilesService;
    /// # async fn example() -> Result<()> {
    /// let client = PortkeyClient::from_env()?;
    ///
    /// let response = client.delete_file("file-abc123").await?;
    /// assert!(response.deleted);
    /// # Ok(())
    /// # }
    /// ```
    fn delete_file(&self, file_id: &str) -> impl Future<Output = Result<DeleteFileResponse>>;
}

impl FilesService for PortkeyClient {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self, request), fields(filename = %request.filename, purpose = %request.purpose))
    )]
    async fn upload_file(&self, request: UploadFileRequest) -> Result<FileObject> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Uploading file"
        );

        let part =
            reqwest::multipart::Part::bytes(request.file).file_name(request.filename.clone());

        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("purpose", request.purpose);

        let response = self
            .post("/files")
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .json::<FileObject>()
            .await?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: crate::TRACING_TARGET_SERVICE,
            id = %response.id,
            filename = %response.filename,
            "File uploaded successfully"
        );

        Ok(response)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    async fn list_files(&self) -> Result<ListFilesResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Listing files"
        );

        let response = self
            .get("/files")
            .send()
            .await?
            .error_for_status()?
            .json::<ListFilesResponse>()
            .await?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: crate::TRACING_TARGET_SERVICE,
            count = response.data.len(),
            "Files listed successfully"
        );

        Ok(response)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), fields(file_id)))]
    async fn retrieve_file(&self, file_id: &str) -> Result<FileObject> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Retrieving file"
        );

        let response = self
            .get(&format!("/files/{}", file_id))
            .send()
            .await?
            .error_for_status()?
            .json::<FileObject>()
            .await?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: crate::TRACING_TARGET_SERVICE,
            id = %response.id,
            filename = %response.filename,
            "File retrieved successfully"
        );

        Ok(response)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), fields(file_id)))]
    async fn retrieve_file_content(&self, file_id: &str) -> Result<Vec<u8>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Retrieving file content"
        );

        let response = self
            .get(&format!("/files/{}/content", file_id))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: crate::TRACING_TARGET_SERVICE,
            size = response.len(),
            "File content retrieved successfully"
        );

        Ok(response.to_vec())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), fields(file_id)))]
    async fn delete_file(&self, file_id: &str) -> Result<DeleteFileResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Deleting file"
        );

        let response = self
            .delete(&format!("/files/{}", file_id))
            .send()
            .await?
            .error_for_status()?
            .json::<DeleteFileResponse>()
            .await?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: crate::TRACING_TARGET_SERVICE,
            id = %response.id,
            deleted = response.deleted,
            "File deleted"
        );

        Ok(response)
    }
}
