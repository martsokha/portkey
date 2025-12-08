use std::future::Future;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{CreateFeedbackRequest, FeedbackResponse, UpdateFeedbackRequest};

/// Service trait for managing feedback.
///
/// This trait provides methods for creating and updating feedback on traces,
/// allowing you to track user satisfaction and model performance.
///
/// # Example
///
/// ```rust,no_run
/// use portkey_sdk::{AuthMethod, PortkeyConfig, FeedbackService, Result};
/// use portkey_sdk::model::{CreateFeedbackRequest, UpdateFeedbackRequest};
/// use std::collections::HashMap;
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
///     // Create feedback
///     let mut metadata = HashMap::new();
///     metadata.insert("user_id".to_string(), serde_json::json!("user123"));
///
///     let create_request = CreateFeedbackRequest {
///         trace_id: "trace-abc-123".to_string(),
///         value: 5,
///         weight: Some(1.0),
///         metadata: Some(metadata),
///     };
///
///     let response = client.create_feedback(create_request).await?;
///     println!("Created feedback: {:?}", response.feedback_ids);
///
///     // Update feedback
///     let feedback_id = &response.feedback_ids[0];
///     let update_request = UpdateFeedbackRequest {
///         value: 8,
///         weight: Some(0.9),
///         metadata: None,
///     };
///
///     let update_response = client.update_feedback(feedback_id, update_request).await?;
///     println!("Updated feedback: {}", update_response.message);
///
///     Ok(())
/// }
/// ```
pub trait FeedbackService {
    /// Creates feedback for a trace.
    ///
    /// # Arguments
    ///
    /// * `request` - The feedback data to create
    ///
    /// # Returns
    ///
    /// Returns a `FeedbackResponse` with the created feedback IDs.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be parsed.
    fn create_feedback(
        &self,
        request: CreateFeedbackRequest,
    ) -> impl Future<Output = Result<FeedbackResponse>>;

    /// Updates existing feedback by ID.
    ///
    /// # Arguments
    ///
    /// * `feedback_id` - The unique identifier of the feedback to update
    /// * `request` - The updated feedback data
    ///
    /// # Returns
    ///
    /// Returns a `FeedbackResponse` confirming the update.
    ///
    /// # Errors
    ///
    /// Returns an error if the feedback is not found or the API request fails.
    fn update_feedback(
        &self,
        feedback_id: &str,
        request: UpdateFeedbackRequest,
    ) -> impl Future<Output = Result<FeedbackResponse>>;
}

impl FeedbackService for PortkeyClient {
    async fn create_feedback(&self, request: CreateFeedbackRequest) -> Result<FeedbackResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            trace_id = %request.trace_id,
            value = request.value,
            "Creating feedback"
        );

        let response = self
            .send_json(reqwest::Method::POST, "/feedback", &request)
            .await?;
        let response = response.error_for_status()?;
        let feedback_response: FeedbackResponse = response.json().await?;

        Ok(feedback_response)
    }

    async fn update_feedback(
        &self,
        feedback_id: &str,
        request: UpdateFeedbackRequest,
    ) -> Result<FeedbackResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            feedback_id = %feedback_id,
            value = request.value,
            "Updating feedback"
        );

        let path = format!("/feedback/{}", feedback_id);
        let response = self
            .send_json(reqwest::Method::PATCH, &path, &request)
            .await?;
        let response = response.error_for_status()?;
        let feedback_response: FeedbackResponse = response.json().await?;

        Ok(feedback_response)
    }
}
