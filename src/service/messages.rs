use std::future::Future;

use crate::model::{
    CreateMessageRequest, ListMessageFilesResponse, ListMessagesResponse, Message, MessageFile,
    ModifyMessageRequest, PaginationParams,
};
use crate::{PortkeyClient, Result};

/// Service for managing messages in threads.
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
///     let message = client.create_message(
///         "thread_abc123",
///         CreateMessageRequest::builder()
///             .role("user")
///             .content("Hello!")
///             .build()
///             .unwrap()
///     ).await?;
///
///     println!("Created message: {}", message.id);
///     Ok(())
/// # }
/// ```
pub trait MessagesService {
    /// Create a message.
    fn create_message(
        &self,
        thread_id: &str,
        request: CreateMessageRequest,
    ) -> impl Future<Output = Result<Message>>;

    /// Retrieve a message.
    fn retrieve_message(
        &self,
        thread_id: &str,
        message_id: &str,
    ) -> impl Future<Output = Result<Message>>;

    /// Modifies a message.
    fn modify_message(
        &self,
        thread_id: &str,
        message_id: &str,
        request: ModifyMessageRequest,
    ) -> impl Future<Output = Result<Message>>;

    /// Returns a list of messages for a given thread.
    fn list_messages(
        &self,
        thread_id: &str,
        params: PaginationParams,
    ) -> impl Future<Output = Result<ListMessagesResponse>>;

    /// Retrieves a message file.
    fn retrieve_message_file(
        &self,
        thread_id: &str,
        message_id: &str,
        file_id: &str,
    ) -> impl Future<Output = Result<MessageFile>>;

    /// Returns a list of message files.
    fn list_message_files(
        &self,
        thread_id: &str,
        message_id: &str,
        params: PaginationParams,
    ) -> impl Future<Output = Result<ListMessageFilesResponse>>;
}

impl MessagesService for PortkeyClient {
    async fn create_message(
        &self,
        thread_id: &str,
        request: CreateMessageRequest,
    ) -> Result<Message> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            "Creating message"
        );

        let response = self
            .send_json(
                reqwest::Method::POST,
                &format!("/threads/{}/messages", thread_id),
                &request,
            )
            .await?;
        let response = response.error_for_status()?;
        let message: Message = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Message created successfully"
        );

        Ok(message)
    }

    async fn retrieve_message(&self, thread_id: &str, message_id: &str) -> Result<Message> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            message_id = %message_id,
            "Retrieving message"
        );

        let response = self
            .send(
                reqwest::Method::GET,
                &format!("/threads/{}/messages/{}", thread_id, message_id),
            )
            .await?;
        let response = response.error_for_status()?;
        let message: Message = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Message retrieved successfully"
        );

        Ok(message)
    }

    async fn modify_message(
        &self,
        thread_id: &str,
        message_id: &str,
        request: ModifyMessageRequest,
    ) -> Result<Message> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            message_id = %message_id,
            "Modifying message"
        );

        let response = self
            .send_json(
                reqwest::Method::POST,
                &format!("/threads/{}/messages/{}", thread_id, message_id),
                &request,
            )
            .await?;
        let response = response.error_for_status()?;
        let message: Message = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Message modified successfully"
        );

        Ok(message)
    }

    async fn list_messages(
        &self,
        thread_id: &str,
        params: PaginationParams<'_>,
    ) -> Result<ListMessagesResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            "Listing messages"
        );

        let query_params = params.to_query_params();
        let query_params_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let response = self
            .send_with_params(
                reqwest::Method::GET,
                &format!("/threads/{}/messages", thread_id),
                &query_params_refs,
            )
            .await?;
        let response = response.error_for_status()?;
        let messages: ListMessagesResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Messages retrieved successfully"
        );

        Ok(messages)
    }

    async fn retrieve_message_file(
        &self,
        thread_id: &str,
        message_id: &str,
        file_id: &str,
    ) -> Result<MessageFile> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            message_id = %message_id,
            file_id = %file_id,
            "Retrieving message file"
        );

        let response = self
            .send(
                reqwest::Method::GET,
                &format!(
                    "/threads/{}/messages/{}/files/{}",
                    thread_id, message_id, file_id
                ),
            )
            .await?;
        let response = response.error_for_status()?;
        let file: MessageFile = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Message file retrieved successfully"
        );

        Ok(file)
    }

    async fn list_message_files(
        &self,
        thread_id: &str,
        message_id: &str,
        params: PaginationParams<'_>,
    ) -> Result<ListMessageFilesResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            thread_id = %thread_id,
            message_id = %message_id,
            "Listing message files"
        );

        let query_params = params.to_query_params();
        let query_params_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let response = self
            .send_with_params(
                reqwest::Method::GET,
                &format!("/threads/{}/messages/{}/files", thread_id, message_id),
                &query_params_refs,
            )
            .await?;
        let response = response.error_for_status()?;
        let files: ListMessageFilesResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Message files retrieved successfully"
        );

        Ok(files)
    }
}
