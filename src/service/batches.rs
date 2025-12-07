use crate::{
    PortkeyClient, Result,
    model::{Batch, CreateBatchRequest, ListBatchesResponse},
};
use std::future::Future;

/// Service for managing batch processing jobs.
///
/// # Example
///
/// ```rust,ignore
/// use portkey::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = PortkeyClient::builder()
///         .api_key("your-api-key")
///         .build()?;
///
///     let batch = client.create_batch(
///         CreateBatchRequest::builder()
///             .input_file_id("file-abc123")
///             .endpoint("/v1/chat/completions")
///             .completion_window("24h")
///             .build()
///             .unwrap()
///     ).await?;
///
///     println!("Created batch: {}", batch.id);
///     Ok(())
/// }
/// ```
pub trait BatchesService {
    /// Creates and executes a batch from an uploaded file of requests.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let batch = client.create_batch(
    ///     CreateBatchRequest::builder()
    ///         .input_file_id("file-abc123")
    ///         .endpoint("/v1/chat/completions")
    ///         .completion_window("24h")
    ///         .build()
    ///         .unwrap()
    /// ).await?;
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
    /// ```rust,ignore
    /// let batch = client.retrieve_batch("batch_abc123").await?;
    /// println!("Status: {}", batch.status);
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
    /// ```rust,ignore
    /// let batch = client.cancel_batch("batch_abc123").await?;
    /// println!("Cancelled batch: {}", batch.id);
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
    /// ```rust,ignore
    /// let batches = client.list_batches(None, Some(10)).await?;
    /// for batch in batches.data {
    ///     println!("Batch {}: {}", batch.id, batch.status);
    /// }
    /// ```
    fn list_batches(
        &self,
        after: Option<&str>,
        limit: Option<i32>,
    ) -> impl Future<Output = Result<ListBatchesResponse>>;
}

impl BatchesService for PortkeyClient {
    async fn create_batch(&self, request: CreateBatchRequest) -> Result<Batch> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Creating batch"
        );

        let response = self.post("/batches").json(&request).send().await?;
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

        let response = self.get(&format!("/batches/{}", batch_id)).send().await?;
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
            .post(&format!("/batches/{}/cancel", batch_id))
            .json(&serde_json::json!({}))
            .send()
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

    async fn list_batches(
        &self,
        after: Option<&str>,
        limit: Option<i32>,
    ) -> Result<ListBatchesResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Listing batches"
        );

        let mut url = "/batches".to_string();
        let mut params = Vec::new();

        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }

        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }

        let response = self.get(&url).send().await?;
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
