use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{
    CreateLogExportRequest, CreateLogExportResponse, DownloadLogExportResponse, ExportTaskResponse,
    InsertLogRequest, InsertLogResponse, ListLogExportsParams, ListLogExportsResponse, LogExport,
    UpdateLogExportRequest, UpdateLogExportResponse,
};
use std::future::Future;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;

/// Service trait for managing log exports.
///
/// This trait provides methods for creating, retrieving, starting, canceling,
/// and downloading log exports from Portkey.
///
/// # Example
///
/// ```rust,no_run
/// use portkey_sdk::{AuthMethod, PortkeyConfig, LogsService, Result};
/// use portkey_sdk::model::{CreateLogExportRequest, GenerationsFilter, LogExportField};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = PortkeyConfig::builder()
///         .with_api_key("your-portkey-api-key")
///         .with_auth_method(AuthMethod::VirtualKey {
///             virtual_key: "your-virtual-key".to_string(),
///         })
///         .build_client()?;
///
///     // Create a log export
///     let filters = GenerationsFilter {
///         time_of_generation_min: Some("2024-10-20".to_string()),
///         time_of_generation_max: Some("2024-10-30".to_string()),
///         ..Default::default()
///     };
///
///     let request = CreateLogExportRequest {
///         workspace_id: Some("workspace-123".to_string()),
///         filters,
///         requested_data: vec![
///             LogExportField::Id,
///             LogExportField::TraceId,
///             LogExportField::CreatedAt,
///         ],
///         description: Some("Export for October 2024".to_string()),
///     };
///
///     let export = client.create_log_export(request).await?;
///     println!("Created export: {}", export.id);
///
///     // Start the export
///     client.start_log_export(&export.id).await?;
///
///     // Retrieve export status
///     let status = client.get_log_export(&export.id).await?;
///     println!("Export status: {:?}", status.status);
///
///     // Download when ready
///     let download = client.download_log_export(&export.id).await?;
///     println!("Download URL: {}", download.signed_url);
///
///     Ok(())
/// }
/// ```
pub trait LogsService {
    /// Creates a new log export.
    ///
    /// # Arguments
    ///
    /// * `request` - The export configuration with filters and requested fields
    ///
    /// # Returns
    ///
    /// Returns a `CreateLogExportResponse` with the export ID and total count.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be parsed.
    fn create_log_export(
        &self,
        request: CreateLogExportRequest,
    ) -> impl Future<Output = Result<CreateLogExportResponse>>;

    /// Retrieves a log export by ID.
    ///
    /// # Arguments
    ///
    /// * `export_id` - The unique identifier of the export
    ///
    /// # Returns
    ///
    /// Returns a `LogExport` with full export details.
    ///
    /// # Errors
    ///
    /// Returns an error if the export is not found or the API request fails.
    fn get_log_export(&self, export_id: &str) -> impl Future<Output = Result<LogExport>>;

    /// Starts processing a log export.
    ///
    /// # Arguments
    ///
    /// * `export_id` - The unique identifier of the export to start
    ///
    /// # Returns
    ///
    /// Returns an `ExportTaskResponse` with status message.
    ///
    /// # Errors
    ///
    /// Returns an error if the export cannot be started or the API request fails.
    fn start_log_export(&self, export_id: &str)
    -> impl Future<Output = Result<ExportTaskResponse>>;

    /// Cancels a running log export.
    ///
    /// # Arguments
    ///
    /// * `export_id` - The unique identifier of the export to cancel
    ///
    /// # Returns
    ///
    /// Returns an `ExportTaskResponse` with status message.
    ///
    /// # Errors
    ///
    /// Returns an error if the export cannot be canceled or the API request fails.
    fn cancel_log_export(
        &self,
        export_id: &str,
    ) -> impl Future<Output = Result<ExportTaskResponse>>;

    /// Downloads a completed log export.
    ///
    /// # Arguments
    ///
    /// * `export_id` - The unique identifier of the export to download
    ///
    /// # Returns
    ///
    /// Returns a `DownloadLogExportResponse` with a pre-signed URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the export is not ready or the API request fails.
    fn download_log_export(
        &self,
        export_id: &str,
    ) -> impl Future<Output = Result<DownloadLogExportResponse>>;

    /// Inserts one or more custom logs.
    ///
    /// # Arguments
    ///
    /// * `request` - Either a single log or multiple logs to insert
    ///
    /// # Returns
    ///
    /// Returns an `InsertLogResponse` with status and log IDs.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be parsed.
    fn insert_log(
        &self,
        request: InsertLogRequest,
    ) -> impl Future<Output = Result<InsertLogResponse>>;

    /// Updates an existing log export.
    ///
    /// # Arguments
    ///
    /// * `export_id` - The unique identifier of the export to update
    /// * `request` - The updated export configuration
    ///
    /// # Returns
    ///
    /// Returns an `UpdateLogExportResponse` with the updated export ID and count.
    ///
    /// # Errors
    ///
    /// Returns an error if the export is not found or the API request fails.
    fn update_log_export(
        &self,
        export_id: &str,
        request: UpdateLogExportRequest,
    ) -> impl Future<Output = Result<UpdateLogExportResponse>>;

    /// Lists all log exports with optional workspace filter.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering exports
    ///
    /// # Returns
    ///
    /// Returns a `ListLogExportsResponse` with all matching exports.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be parsed.
    fn list_log_exports(
        &self,
        params: Option<ListLogExportsParams>,
    ) -> impl Future<Output = Result<ListLogExportsResponse>>;
}

impl LogsService for PortkeyClient {
    async fn create_log_export(
        &self,
        request: CreateLogExportRequest,
    ) -> Result<CreateLogExportResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            workspace_id = ?request.workspace_id,
            requested_data_count = request.requested_data.len(),
            "Creating log export"
        );

        let response = self.post("/logs/exports").json(&request).send().await?;
        let response = response.error_for_status()?;
        let export_response: CreateLogExportResponse = response.json().await?;

        Ok(export_response)
    }

    async fn get_log_export(&self, export_id: &str) -> Result<LogExport> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            export_id = %export_id,
            "Retrieving log export"
        );

        let path = format!("/logs/exports/{}", export_id);
        let response = self.get(&path).send().await?;
        let response = response.error_for_status()?;
        let export: LogExport = response.json().await?;

        Ok(export)
    }

    async fn start_log_export(&self, export_id: &str) -> Result<ExportTaskResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            export_id = %export_id,
            "Starting log export"
        );

        let path = format!("/logs/exports/{}/start", export_id);
        let response = self.post(&path).send().await?;
        let response = response.error_for_status()?;
        let task_response: ExportTaskResponse = response.json().await?;

        Ok(task_response)
    }

    async fn cancel_log_export(&self, export_id: &str) -> Result<ExportTaskResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            export_id = %export_id,
            "Canceling log export"
        );

        let path = format!("/logs/exports/{}/cancel", export_id);
        let response = self.post(&path).send().await?;
        let response = response.error_for_status()?;
        let task_response: ExportTaskResponse = response.json().await?;

        Ok(task_response)
    }

    async fn download_log_export(&self, export_id: &str) -> Result<DownloadLogExportResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            export_id = %export_id,
            "Downloading log export"
        );

        let path = format!("/logs/exports/{}/download", export_id);
        let response = self.get(&path).send().await?;
        let response = response.error_for_status()?;
        let download_response: DownloadLogExportResponse = response.json().await?;

        Ok(download_response)
    }

    async fn insert_log(&self, request: InsertLogRequest) -> Result<InsertLogResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            "Inserting custom log(s)"
        );

        let response = self.post("/logs").json(&request).send().await?;
        let response = response.error_for_status()?;
        let insert_response: InsertLogResponse = response.json().await?;

        Ok(insert_response)
    }

    async fn update_log_export(
        &self,
        export_id: &str,
        request: UpdateLogExportRequest,
    ) -> Result<UpdateLogExportResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            export_id = %export_id,
            workspace_id = ?request.workspace_id,
            "Updating log export"
        );

        let path = format!("/logs/exports/{}", export_id);
        let response = self.put(&path).json(&request).send().await?;
        let response = response.error_for_status()?;
        let update_response: UpdateLogExportResponse = response.json().await?;

        Ok(update_response)
    }

    async fn list_log_exports(
        &self,
        params: Option<ListLogExportsParams>,
    ) -> Result<ListLogExportsResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            workspace_id = ?params.as_ref().and_then(|p| p.workspace_id.as_ref()),
            "Listing log exports"
        );

        let mut request = self.get("/logs/exports");

        if let Some(p) = params
            && let Some(workspace_id) = p.workspace_id
        {
            request = request.query(&[("workspace_id", workspace_id)]);
        }

        let response = request.send().await?;
        let response = response.error_for_status()?;
        let list_response: ListLogExportsResponse = response.json().await?;

        Ok(list_response)
    }
}
