use std::future::Future;

use crate::model::{
    Assistant, AssistantFile, CreateAssistantFileRequest, CreateAssistantRequest,
    DeleteAssistantFileResponse, DeleteAssistantResponse, ListAssistantFilesResponse,
    ListAssistantsResponse, ModifyAssistantRequest, PaginationParams,
};
use crate::{PortkeyClient, Result};

/// Service for managing assistants.
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
///     let assistant = client.create_assistant(
///         CreateAssistantRequest::builder()
///             .model("gpt-4")
///             .name("Math Tutor")
///             .instructions("You are a helpful math tutor.")
///             .build()
///             .unwrap()
///     ).await?;
///
///     println!("Created assistant: {}", assistant.id);
///     Ok(())
/// # }
/// ```
pub trait AssistantsService {
    /// Create an assistant with a model and instructions.
    fn create_assistant(
        &self,
        request: CreateAssistantRequest,
    ) -> impl Future<Output = Result<Assistant>>;

    /// Retrieves an assistant.
    fn retrieve_assistant(&self, assistant_id: &str) -> impl Future<Output = Result<Assistant>>;

    /// Modifies an assistant.
    fn modify_assistant(
        &self,
        assistant_id: &str,
        request: ModifyAssistantRequest,
    ) -> impl Future<Output = Result<Assistant>>;

    /// Delete an assistant.
    fn delete_assistant(
        &self,
        assistant_id: &str,
    ) -> impl Future<Output = Result<DeleteAssistantResponse>>;

    /// Returns a list of assistants.
    fn list_assistants(
        &self,
        params: PaginationParams,
    ) -> impl Future<Output = Result<ListAssistantsResponse>>;

    /// Create an assistant file by attaching a File to an assistant.
    fn create_assistant_file(
        &self,
        assistant_id: &str,
        request: CreateAssistantFileRequest,
    ) -> impl Future<Output = Result<AssistantFile>>;

    /// Retrieves an AssistantFile.
    fn retrieve_assistant_file(
        &self,
        assistant_id: &str,
        file_id: &str,
    ) -> impl Future<Output = Result<AssistantFile>>;

    /// Delete an assistant file.
    fn delete_assistant_file(
        &self,
        assistant_id: &str,
        file_id: &str,
    ) -> impl Future<Output = Result<DeleteAssistantFileResponse>>;

    /// Returns a list of assistant files.
    fn list_assistant_files(
        &self,
        assistant_id: &str,
        params: PaginationParams,
    ) -> impl Future<Output = Result<ListAssistantFilesResponse>>;
}

impl AssistantsService for PortkeyClient {
    async fn create_assistant(&self, request: CreateAssistantRequest) -> Result<Assistant> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Creating assistant"
        );

        let response = self.post("/assistants").json(&request).send().await?;
        let response = response.error_for_status()?;
        let assistant: Assistant = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Assistant created successfully"
        );

        Ok(assistant)
    }

    async fn retrieve_assistant(&self, assistant_id: &str) -> Result<Assistant> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            assistant_id = %assistant_id,
            "Retrieving assistant"
        );

        let response = self
            .get(&format!("/assistants/{}", assistant_id))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let assistant: Assistant = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Assistant retrieved successfully"
        );

        Ok(assistant)
    }

    async fn modify_assistant(
        &self,
        assistant_id: &str,
        request: ModifyAssistantRequest,
    ) -> Result<Assistant> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            assistant_id = %assistant_id,
            "Modifying assistant"
        );

        let response = self
            .post(&format!("/assistants/{}", assistant_id))
            .json(&request)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let assistant: Assistant = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Assistant modified successfully"
        );

        Ok(assistant)
    }

    async fn delete_assistant(&self, assistant_id: &str) -> Result<DeleteAssistantResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            assistant_id = %assistant_id,
            "Deleting assistant"
        );

        let response = self
            .delete(&format!("/assistants/{}", assistant_id))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let delete_response: DeleteAssistantResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Assistant deleted successfully"
        );

        Ok(delete_response)
    }

    async fn list_assistants(
        &self,
        params: PaginationParams<'_>,
    ) -> Result<ListAssistantsResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Listing assistants"
        );

        let query_params = params.to_query_params();
        let query_params_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let url = self.build_url("/assistants", &query_params_refs);

        let response = self.get(url.as_str()).send().await?;
        let response = response.error_for_status()?;
        let assistants: ListAssistantsResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Assistants retrieved successfully"
        );

        Ok(assistants)
    }

    async fn create_assistant_file(
        &self,
        assistant_id: &str,
        request: CreateAssistantFileRequest,
    ) -> Result<AssistantFile> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            assistant_id = %assistant_id,
            "Creating assistant file"
        );

        let response = self
            .post(&format!("/assistants/{}/files", assistant_id))
            .json(&request)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let file: AssistantFile = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Assistant file created successfully"
        );

        Ok(file)
    }

    async fn retrieve_assistant_file(
        &self,
        assistant_id: &str,
        file_id: &str,
    ) -> Result<AssistantFile> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            assistant_id = %assistant_id,
            file_id = %file_id,
            "Retrieving assistant file"
        );

        let response = self
            .get(&format!("/assistants/{}/files/{}", assistant_id, file_id))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let file: AssistantFile = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Assistant file retrieved successfully"
        );

        Ok(file)
    }

    async fn delete_assistant_file(
        &self,
        assistant_id: &str,
        file_id: &str,
    ) -> Result<DeleteAssistantFileResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            assistant_id = %assistant_id,
            file_id = %file_id,
            "Deleting assistant file"
        );

        let response = self
            .delete(&format!("/assistants/{}/files/{}", assistant_id, file_id))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let delete_response: DeleteAssistantFileResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Assistant file deleted successfully"
        );

        Ok(delete_response)
    }

    async fn list_assistant_files(
        &self,
        assistant_id: &str,
        params: PaginationParams<'_>,
    ) -> Result<ListAssistantFilesResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            assistant_id = %assistant_id,
            "Listing assistant files"
        );

        let query_params = params.to_query_params();
        let query_params_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let url = self.build_url(
            &format!("/assistants/{}/files", assistant_id),
            &query_params_refs,
        );

        let response = self.get(url.as_str()).send().await?;
        let response = response.error_for_status()?;
        let files: ListAssistantFilesResponse = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: crate::TRACING_TARGET_SERVICE,
            "Assistant files retrieved successfully"
        );

        Ok(files)
    }
}
