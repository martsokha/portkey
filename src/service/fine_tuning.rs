use std::future::Future;

use crate::model::{
    CreateFineTuningJobRequest, FineTuningJob, ListFineTuningJobCheckpointsResponse,
    ListFineTuningJobEventsResponse, ListFineTuningJobsResponse, PaginationParams,
};
use crate::{PortkeyClient, Result};

/// Service for managing fine-tuning jobs.
///
/// # Example
///
/// ```no_run
/// # use portkey_sdk::{PortkeyConfig, PortkeyClient, Result};
/// # use portkey_sdk::service::FineTuningService;
/// # use portkey_sdk::model::CreateFineTuningJobRequest;
/// # async fn example() -> Result<()> {
/// let config = PortkeyConfig::builder()
///     .with_api_key("your-api-key")
///     .build()?;
/// let client = PortkeyClient::new(config)?;
///
/// let job = client.create_fine_tuning_job(
///     CreateFineTuningJobRequest {
///         model: "gpt-3.5-turbo".to_string(),
///         training_file: "file-abc123".to_string(),
///         ..Default::default()
///     }
/// ).await?;
///
/// println!("Created fine-tuning job: {}", job.id);
/// # Ok(())
/// # }
/// ```
pub trait FineTuningService {
    /// Creates a fine-tuning job which begins the process of creating a new model from a given dataset.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyConfig, PortkeyClient, Result};
    /// # use portkey_sdk::service::FineTuningService;
    /// # use portkey_sdk::model::CreateFineTuningJobRequest;
    /// # async fn example(client: PortkeyClient) -> Result<()> {
    /// let job = client.create_fine_tuning_job(
    ///     CreateFineTuningJobRequest {
    ///         model: "gpt-3.5-turbo".to_string(),
    ///         training_file: "file-abc123".to_string(),
    ///         suffix: Some("my-custom-model".to_string()),
    ///         ..Default::default()
    ///     }
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn create_fine_tuning_job(
        &self,
        request: CreateFineTuningJobRequest,
    ) -> impl Future<Output = Result<FineTuningJob>>;

    /// List your organization's fine-tuning jobs.
    ///
    /// # Arguments
    ///
    /// * `after` - Identifier for the last job from the previous pagination request.
    /// * `limit` - Number of fine-tuning jobs to retrieve (default: 20).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FineTuningService;
    /// # async fn example(client: PortkeyClient) -> Result<()> {
    /// let jobs = client.list_fine_tuning_jobs(None, Some(10)).await?;
    /// for job in jobs.data {
    ///     println!("Job {}: {}", job.id, job.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn list_fine_tuning_jobs(
        &self,
        params: PaginationParams,
    ) -> impl Future<Output = Result<ListFineTuningJobsResponse>>;

    /// Get info about a fine-tuning job.
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FineTuningService;
    /// # async fn example(client: PortkeyClient) -> Result<()> {
    /// let job = client.retrieve_fine_tuning_job("ftjob-abc123").await?;
    /// println!("Status: {}", job.status);
    /// # Ok(())
    /// # }
    /// ```
    fn retrieve_fine_tuning_job(
        &self,
        fine_tuning_job_id: &str,
    ) -> impl Future<Output = Result<FineTuningJob>>;

    /// Immediately cancel a fine-tuning job.
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job to cancel.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FineTuningService;
    /// # async fn example(client: PortkeyClient) -> Result<()> {
    /// let job = client.cancel_fine_tuning_job("ftjob-abc123").await?;
    /// println!("Cancelled job: {}", job.id);
    /// # Ok(())
    /// # }
    /// ```
    fn cancel_fine_tuning_job(
        &self,
        fine_tuning_job_id: &str,
    ) -> impl Future<Output = Result<FineTuningJob>>;

    /// Get status updates for a fine-tuning job.
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job to get events for.
    /// * `after` - Identifier for the last event from the previous pagination request.
    /// * `limit` - Number of events to retrieve (default: 20).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FineTuningService;
    /// # async fn example(client: PortkeyClient) -> Result<()> {
    /// let events = client.list_fine_tuning_job_events("ftjob-abc123", None, Some(10)).await?;
    /// for event in events.data {
    ///     println!("[{}] {}", event.level, event.message);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn list_fine_tuning_job_events(
        &self,
        fine_tuning_job_id: &str,
        params: PaginationParams,
    ) -> impl Future<Output = Result<ListFineTuningJobEventsResponse>>;

    /// List checkpoints for a fine-tuning job.
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job to get checkpoints for.
    /// * `after` - Identifier for the last checkpoint from the previous pagination request.
    /// * `limit` - Number of checkpoints to retrieve (default: 10).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::{PortkeyClient, Result};
    /// # use portkey_sdk::service::FineTuningService;
    /// # async fn example(client: PortkeyClient) -> Result<()> {
    /// let checkpoints = client.list_fine_tuning_job_checkpoints("ftjob-abc123", None, Some(5)).await?;
    /// for checkpoint in checkpoints.data {
    ///     println!("Checkpoint at step {}", checkpoint.step_number);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn list_fine_tuning_job_checkpoints(
        &self,
        fine_tuning_job_id: &str,
        params: PaginationParams,
    ) -> impl Future<Output = Result<ListFineTuningJobCheckpointsResponse>>;
}

impl FineTuningService for PortkeyClient {
    async fn create_fine_tuning_job(
        &self,
        request: CreateFineTuningJobRequest,
    ) -> Result<FineTuningJob> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Creating fine-tuning job"
        );

        let response = self
            .post("/fine_tuning/jobs")?
            .json(&request)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let job: FineTuningJob = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Fine-tuning job created successfully"
        );

        Ok(job)
    }

    async fn list_fine_tuning_jobs(
        &self,
        params: PaginationParams<'_>,
    ) -> Result<ListFineTuningJobsResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Listing fine-tuning jobs"
        );

        let query_params = params.to_query_params();
        let query_params_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let url = self.build_url("/fine_tuning/jobs", &query_params_refs);

        let response = self.get(url?.as_str())?.send().await?;
        let response = response.error_for_status()?;
        let jobs: ListFineTuningJobsResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Fine-tuning jobs retrieved successfully"
        );

        Ok(jobs)
    }

    async fn retrieve_fine_tuning_job(&self, fine_tuning_job_id: &str) -> Result<FineTuningJob> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            job_id = %fine_tuning_job_id,
            "Retrieving fine-tuning job"
        );

        let response = self
            .get(&format!("/fine_tuning/jobs/{}", fine_tuning_job_id))?
            .send()
            .await?;
        let response = response.error_for_status()?;
        let job: FineTuningJob = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Fine-tuning job retrieved successfully"
        );

        Ok(job)
    }

    async fn cancel_fine_tuning_job(&self, fine_tuning_job_id: &str) -> Result<FineTuningJob> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            job_id = %fine_tuning_job_id,
            "Cancelling fine-tuning job"
        );

        let response = self
            .post(&format!("/fine_tuning/jobs/{}/cancel", fine_tuning_job_id))?
            .json(&serde_json::json!({}))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let job: FineTuningJob = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Fine-tuning job cancelled successfully"
        );

        Ok(job)
    }

    async fn list_fine_tuning_job_events(
        &self,
        fine_tuning_job_id: &str,
        params: PaginationParams<'_>,
    ) -> Result<ListFineTuningJobEventsResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            job_id = %fine_tuning_job_id,
            "Listing fine-tuning job events"
        );

        let query_params = params.to_query_params();
        let query_params_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let url = self.build_url(
            &format!("/fine_tuning/jobs/{}/events", fine_tuning_job_id),
            &query_params_refs,
        );

        let response = self.get(url?.as_str())?.send().await?;
        let response = response.error_for_status()?;
        let events: ListFineTuningJobEventsResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Fine-tuning job events retrieved successfully"
        );

        Ok(events)
    }

    async fn list_fine_tuning_job_checkpoints(
        &self,
        fine_tuning_job_id: &str,
        params: PaginationParams<'_>,
    ) -> Result<ListFineTuningJobCheckpointsResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            job_id = %fine_tuning_job_id,
            "Listing fine-tuning job checkpoints"
        );

        let query_params = params.to_query_params();
        let query_params_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let url = self.build_url(
            &format!("/fine_tuning/jobs/{}/checkpoints", fine_tuning_job_id),
            &query_params_refs,
        );

        let response = self.get(url?.as_str())?.send().await?;
        let response = response.error_for_status()?;
        let checkpoints: ListFineTuningJobCheckpointsResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Fine-tuning job checkpoints retrieved successfully"
        );

        Ok(checkpoints)
    }
}
