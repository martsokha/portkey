use std::future::Future;

use crate::model::{CreateThreadRequest, DeleteThreadResponse, ModifyThreadRequest, Thread};
use crate::{PortkeyClient, Result};

/// Service for managing threads.
///
/// # Example
///
/// ```no_run
/// # use portkey_sdk::{PortkeyConfig, PortkeyClient, Result};
/// # async fn example() -> Result<()> {
/// let config = PortkeyConfig::builder()
///     .with_api_key("your-api-key")
///     .build()?;
/// let client = PortkeyClient::new(config)?;
///
///     let thread = client.create_thread(
///         CreateThreadRequest::builder()
///             .build()
///             .unwrap()
///     ).await?;
///
///     println!("Created thread: {}", thread.id);
///     Ok(())
/// # }
/// ```
pub trait ThreadsService {
    /// Create a thread.
    fn create_thread(&self, request: CreateThreadRequest) -> impl Future<Output = Result<Thread>>;

    /// Retrieves a thread.
    fn retrieve_thread(&self, thread_id: &str) -> impl Future<Output = Result<Thread>>;

    /// Modifies a thread.
    fn modify_thread(
        &self,
        thread_id: &str,
        request: ModifyThreadRequest,
    ) -> impl Future<Output = Result<Thread>>;

    /// Delete a thread.
    fn delete_thread(&self, thread_id: &str) -> impl Future<Output = Result<DeleteThreadResponse>>;
}

impl ThreadsService for PortkeyClient {
    async fn create_thread(&self, request: CreateThreadRequest) -> Result<Thread> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Creating thread"
        );

        let response = self.post("/threads").json(&request).send().await?;
        let response = response.error_for_status()?;
        let thread: Thread = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Thread created successfully"
        );

        Ok(thread)
    }

    async fn retrieve_thread(&self, thread_id: &str) -> Result<Thread> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            "Retrieving thread"
        );

        let response = self.get(&format!("/threads/{}", thread_id)).send().await?;
        let response = response.error_for_status()?;
        let thread: Thread = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Thread retrieved successfully"
        );

        Ok(thread)
    }

    async fn modify_thread(&self, thread_id: &str, request: ModifyThreadRequest) -> Result<Thread> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            "Modifying thread"
        );

        let response = self
            .post(&format!("/threads/{}", thread_id))
            .json(&request)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let thread: Thread = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Thread modified successfully"
        );

        Ok(thread)
    }

    async fn delete_thread(&self, thread_id: &str) -> Result<DeleteThreadResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            "Deleting thread"
        );

        let response = self
            .delete(&format!("/threads/{}", thread_id))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let delete_response: DeleteThreadResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Thread deleted successfully"
        );

        Ok(delete_response)
    }
}
