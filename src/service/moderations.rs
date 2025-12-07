use crate::{
    PortkeyClient, Result,
    model::{CreateModerationRequest, ModerationResponse},
};
use std::future::Future;

/// Service for moderating content.
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
///     let response = client.create_moderation(
///         CreateModerationRequest::builder()
///             .input("I want to hurt someone")
///             .build()
///             .unwrap()
///     ).await?;
///
///     for result in response.results {
///         if result.flagged {
///             println!("Content was flagged!");
///         }
///     }
///     Ok(())
/// }
/// ```
pub trait ModerationsService {
    /// Classifies if text is potentially harmful.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let response = client.create_moderation(
    ///     CreateModerationRequest::builder()
    ///         .input("Sample text to moderate")
    ///         .model("text-moderation-latest")
    ///         .build()
    ///         .unwrap()
    /// ).await?;
    ///
    /// for result in response.results {
    ///     if result.flagged {
    ///         println!("Flagged categories:");
    ///         if result.categories.hate {
    ///             println!("  - Hate (score: {})", result.category_scores.hate);
    ///         }
    ///         if result.categories.violence {
    ///             println!("  - Violence (score: {})", result.category_scores.violence);
    ///         }
    ///     }
    /// }
    /// ```
    fn create_moderation(
        &self,
        request: CreateModerationRequest,
    ) -> impl Future<Output = Result<ModerationResponse>>;
}

impl ModerationsService for PortkeyClient {
    async fn create_moderation(
        &self,
        request: CreateModerationRequest,
    ) -> Result<ModerationResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Creating moderation"
        );

        let response = self.post("/moderations").json(&request).send().await?;
        let response = response.error_for_status()?;
        let moderation: ModerationResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Moderation created successfully"
        );

        Ok(moderation)
    }
}
