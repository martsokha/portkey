use std::future::Future;

use crate::model::{CreateModerationRequest, ModerationResponse};
use crate::{PortkeyClient, Result};

/// Service for moderating content.
///
/// # Example
///
/// ```no_run
/// use portkey_sdk::{PortkeyConfig, PortkeyClient, Result};
/// use portkey_sdk::service::ModerationsService;
/// use portkey_sdk::model::CreateModerationRequest;
///
/// # async fn example() -> Result<()> {
/// let config = PortkeyConfig::builder()
///     .with_api_key("your-api-key")
///     .build()?;
/// let client = PortkeyClient::new(config)?;
///
///     let response = client.create_moderation(
///         CreateModerationRequest {
///             input: portkey_sdk::model::ModerationInput::String("I want to hurt someone".to_string()),
///             model: None,
///         }
///     ).await?;
///
///     for result in response.results {
///         if result.flagged {
///             println!("Content was flagged!");
///         }
///     }
///     Ok(())
/// # }
/// ```
pub trait ModerationsService {
    /// Classifies if text is potentially harmful.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use portkey_sdk::model::{CreateModerationRequest, ModerationInput};
    /// # use portkey_sdk::service::ModerationsService;
    /// # async fn example(client: &impl ModerationsService) -> portkey_sdk::Result<()> {
    /// let response = client.create_moderation(
    ///     CreateModerationRequest {
    ///         input: ModerationInput::String("Sample text to moderate".to_string()),
    ///         model: Some("text-moderation-latest".to_string()),
    ///     }
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
    /// # Ok(())
    /// # }
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

        let response = self
            .send_json(reqwest::Method::POST, "/moderations", &request)
            .await?;
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
