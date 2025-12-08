use std::future::Future;

use crate::model::{Batch, CreateBatchRequest, ListBatchesResponse, PaginationParams};
use crate::{PortkeyClient, Result};

/// Service for managing batch processing jobs.
///
/// # Example
///
/// ```no_run
/// # use portkey_sdk::{PortkeyConfig, PortkeyClient, Result};
/// # use portkey_sdk::service::BatchesService;
/// # use portkey_sdk::model::CreateBatchRequest;
/// # async fn example() -> Result<()> {
/// let config = PortkeyConfig::builder()
///     .with_api_key("your-api-key")
///     .build()?;
/// let client = PortkeyClient::new(config)?;
///
/// let batch = client.create_batch(
///     CreateBatchRequest {
///         input_file_id: "file-abc123".to_string(),
///         endpoint: "/v1/chat/completions".to_string(),
///         completion_window: "24h".to_string(),
///         metadata: None,
///     }
/// ).await?;
///
/// println!("Created batch: {}", batch.id);
/// # Ok(())
/// # }
/// ```
pub trait BatchesService {
    /// Creates and executes a batch from an uploaded file of requests.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::BatchesService;
    /// # use portkey_sdk::model::CreateBatchRequest;
    /// # async fn example(client: PortkeyClient) -> Result<()> {
    /// let batch = client.create_batch(
    ///     CreateBatchRequest {
    ///         input_file_id: "file-abc123".to_string(),
    ///         endpoint: "/v1/chat/completions".to_string(),
    ///         completion_window: "24h".to_string(),
    ///         metadata: None,
    ///     }
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn create_batch(&self, request: CreateBatchRequest) -> impl Future<Output = Result<Batch>>;

    /// Retrieves a batch.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - The ID of the batch to retrieve.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::BatchesService;
    /// # async fn example(client: PortkeyClient) -> Result<()> {
    /// let batch = client.retrieve_batch("batch_abc123").await?;
    /// println!("Status: {}", batch.status);
    /// # Ok(())
    /// # }
    /// ```
    fn retrieve_batch(&self, batch_id: &str) -> impl Future<Output = Result<Batch>>;

    /// Cancels an in-progress batch.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - The ID of the batch to cancel.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::BatchesService;
    /// # async fn example(client: PortkeyClient) -> Result<()> {
    /// let batch = client.cancel_batch("batch_abc123").await?;
    /// println!("Cancelled batch: {}", batch.id);
    /// # Ok(())
    /// # }
    /// ```
    fn cancel_batch(&self, batch_id: &str) -> impl Future<Output = Result<Batch>>;

    /// List your organization's batches.
    ///
    /// # Arguments
    ///
    /// * `after` - A cursor for use in pagination.
    /// * `limit` - A limit on the number of objects to be returned (1-100, default: 20).
    ///
    /// # Example
    ///
    /// ```no_run
    /// let batches = client.list_batches(None, Some(10)).await?;
    /// for batch in batches.data {
    ///     println!("Batch {}: {}", batch.id, batch.status);
    /// # }
    /// ```
    fn list_batches(
        &self,
        params: PaginationParams,
    ) -> impl Future<Output = Result<ListBatchesResponse>>;
}

impl BatchesService for PortkeyClient {
    async fn create_batch(&self, request: CreateBatchRequest) -> Result<Batch> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Creating batch"
        );

        let response = self
            .send_json(reqwest::Method::POST, "/batches", &request)
            .await?;
        let response = response.error_for_status()?;
        let batch: Batch = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Batch created successfully"
        );

        Ok(batch)
    }

    async fn retrieve_batch(&self, batch_id: &str) -> Result<Batch> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            batch_id = %batch_id,
            "Retrieving batch"
        );

        let response = self
            .send(reqwest::Method::GET, &format!("/batches/{}", batch_id))
            .await?;
        let response = response.error_for_status()?;
        let batch: Batch = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Batch retrieved successfully"
        );

        Ok(batch)
    }

    async fn cancel_batch(&self, batch_id: &str) -> Result<Batch> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            batch_id = %batch_id,
            "Cancelling batch"
        );

        let response = self
            .send_json(
                reqwest::Method::POST,
                &format!("/batches/{}/cancel", batch_id),
                &serde_json::json!({}),
            )
            .await?;
        let response = response.error_for_status()?;
        let batch: Batch = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Batch cancelled successfully"
        );

        Ok(batch)
    }

    async fn list_batches(&self, params: PaginationParams<'_>) -> Result<ListBatchesResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Listing batches"
        );

        let query_params = params.to_query_params();
        let query_params_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let response = self
            .send_with_params(reqwest::Method::GET, "/batches", &query_params_refs)
            .await?;
        let response = response.error_for_status()?;
        let batches: ListBatchesResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Batches retrieved successfully"
        );

        Ok(batches)
    }
}
