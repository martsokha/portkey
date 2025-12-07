use crate::{
    PortkeyClient, Result,
    model::{
        CreateRunRequest, ListRunStepsResponse, ListRunsResponse, ModifyRunRequest, Run, RunStep,
        SubmitToolOutputsRequest,
    },
};
use std::future::Future;

/// Service for managing runs.
///
/// # Example
///
/// ```no_run
/// # use portkey_sdk::{PortkeyConfig, PortkeyClient, Result};
///

/// # async fn example() -> Result<()> {
/// let config = PortkeyConfig::builder()
///     .with_api_key("your-api-key")
///     .build()?;
/// let client = PortkeyClient::new(config)?;
///
///     let run = client.create_run(
///         "thread_abc123",
///         CreateRunRequest::builder()
///             .assistant_id("asst_abc123")
///             .build()
///             .unwrap()
///     ).await?;
///
///     println!("Created run: {}", run.id);
///     Ok(())
/// # }
/// ```
pub trait RunsService {
    /// Create a run.
    fn create_run(
        &self,
        thread_id: &str,
        request: CreateRunRequest,
    ) -> impl Future<Output = Result<Run>>;

    /// Retrieves a run.
    fn retrieve_run(&self, thread_id: &str, run_id: &str) -> impl Future<Output = Result<Run>>;

    /// Modifies a run.
    fn modify_run(
        &self,
        thread_id: &str,
        run_id: &str,
        request: ModifyRunRequest,
    ) -> impl Future<Output = Result<Run>>;

    /// Returns a list of runs belonging to a thread.
    fn list_runs(
        &self,
        thread_id: &str,
        limit: Option<i32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> impl Future<Output = Result<ListRunsResponse>>;

    /// When a run has the status: "requires_action" and required_action.type is submit_tool_outputs,
    /// this endpoint can be used to submit the outputs from the tool calls once they're all completed.
    fn submit_tool_outputs(
        &self,
        thread_id: &str,
        run_id: &str,
        request: SubmitToolOutputsRequest,
    ) -> impl Future<Output = Result<Run>>;

    /// Cancels a run that is in_progress.
    fn cancel_run(&self, thread_id: &str, run_id: &str) -> impl Future<Output = Result<Run>>;

    /// Create a thread and run it in one request.
    fn create_thread_and_run(&self, request: CreateRunRequest)
    -> impl Future<Output = Result<Run>>;

    /// Retrieves a run step.
    fn retrieve_run_step(
        &self,
        thread_id: &str,
        run_id: &str,
        step_id: &str,
    ) -> impl Future<Output = Result<RunStep>>;

    /// Returns a list of run steps belonging to a run.
    fn list_run_steps(
        &self,
        thread_id: &str,
        run_id: &str,
        limit: Option<i32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> impl Future<Output = Result<ListRunStepsResponse>>;
}

impl RunsService for PortkeyClient {
    async fn create_run(&self, thread_id: &str, request: CreateRunRequest) -> Result<Run> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            "Creating run"
        );

        let response = self
            .post(&format!("/threads/{}/runs", thread_id))
            .json(&request)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let run: Run = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Run created successfully"
        );

        Ok(run)
    }

    async fn retrieve_run(&self, thread_id: &str, run_id: &str) -> Result<Run> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            run_id = %run_id,
            "Retrieving run"
        );

        let response = self
            .get(&format!("/threads/{}/runs/{}", thread_id, run_id))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let run: Run = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Run retrieved successfully"
        );

        Ok(run)
    }

    async fn modify_run(
        &self,
        thread_id: &str,
        run_id: &str,
        request: ModifyRunRequest,
    ) -> Result<Run> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            run_id = %run_id,
            "Modifying run"
        );

        let response = self
            .post(&format!("/threads/{}/runs/{}", thread_id, run_id))
            .json(&request)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let run: Run = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Run modified successfully"
        );

        Ok(run)
    }

    async fn list_runs(
        &self,
        thread_id: &str,
        limit: Option<i32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> Result<ListRunsResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            "Listing runs"
        );

        let mut url = format!("/threads/{}/runs", thread_id);
        let mut params = Vec::new();

        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(order) = order {
            params.push(format!("order={}", order));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(before) = before {
            params.push(format!("before={}", before));
        }

        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }

        let response = self.get(&url).send().await?;
        let response = response.error_for_status()?;
        let runs: ListRunsResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Runs retrieved successfully"
        );

        Ok(runs)
    }

    async fn submit_tool_outputs(
        &self,
        thread_id: &str,
        run_id: &str,
        request: SubmitToolOutputsRequest,
    ) -> Result<Run> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            run_id = %run_id,
            "Submitting tool outputs"
        );

        let response = self
            .post(&format!(
                "/threads/{}/runs/{}/submit_tool_outputs",
                thread_id, run_id
            ))
            .json(&request)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let run: Run = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Tool outputs submitted successfully"
        );

        Ok(run)
    }

    async fn cancel_run(&self, thread_id: &str, run_id: &str) -> Result<Run> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            run_id = %run_id,
            "Cancelling run"
        );

        let response = self
            .post(&format!("/threads/{}/runs/{}/cancel", thread_id, run_id))
            .json(&serde_json::json!({}))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let run: Run = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Run cancelled successfully"
        );

        Ok(run)
    }

    async fn create_thread_and_run(&self, request: CreateRunRequest) -> Result<Run> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Creating thread and run"
        );

        let response = self.post("/threads/runs").json(&request).send().await?;
        let response = response.error_for_status()?;
        let run: Run = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Thread and run created successfully"
        );

        Ok(run)
    }

    async fn retrieve_run_step(
        &self,
        thread_id: &str,
        run_id: &str,
        step_id: &str,
    ) -> Result<RunStep> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            run_id = %run_id,
            step_id = %step_id,
            "Retrieving run step"
        );

        let response = self
            .get(&format!(
                "/threads/{}/runs/{}/steps/{}",
                thread_id, run_id, step_id
            ))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let step: RunStep = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Run step retrieved successfully"
        );

        Ok(step)
    }

    async fn list_run_steps(
        &self,
        thread_id: &str,
        run_id: &str,
        limit: Option<i32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> Result<ListRunStepsResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            run_id = %run_id,
            "Listing run steps"
        );

        let mut url = format!("/threads/{}/runs/{}/steps", thread_id, run_id);
        let mut params = Vec::new();

        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(order) = order {
            params.push(format!("order={}", order));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(before) = before {
            params.push(format!("before={}", before));
        }

        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }

        let response = self.get(&url).send().await?;
        let response = response.error_for_status()?;
        let steps: ListRunStepsResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Run steps retrieved successfully"
        );

        Ok(steps)
    }
}
