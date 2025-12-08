use std::future::Future;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{CreateResponseRequest, ListInputItemsParams, ListInputItemsResponse, Response};

/// Service trait for managing responses in Portkey.
///
/// This trait provides methods for creating, retrieving, listing, and deleting
/// responses, as well as managing input items associated with responses.
///
/// # Example
///
/// ```rust,no_run
/// use portkey_sdk::{AuthMethod, PortkeyConfig, ResponsesService, Result};
/// use portkey_sdk::model::CreateResponseRequest;
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
///     // Create a new response
///     let request = CreateResponseRequest {
///         trace_id: Some("trace-123".to_string()),
///         model: Some("gpt-4".to_string()),
///         provider: Some("openai".to_string()),
///         status: Some("success".to_string()),
///         total_tokens: Some(150),
///         prompt_tokens: Some(100),
///         completion_tokens: Some(50),
///         latency_ms: Some(1500),
///         cost: Some(0.003),
///         metadata: None,
///         request: None,
///         response: None,
///     };
///
///     let response = client.create_response(request).await?;
///     println!("Created response: {}", response.id);
///
///     // Get the response
///     let fetched = client.get_response(&response.id).await?;
///     println!("Fetched response: {:?}", fetched);
///
///     // List input items for the response
///     let params = portkey_sdk::model::ListInputItemsParams {
///         limit: Some(50),
///         offset: Some(0),
///     };
///     let input_items = client.list_input_items(&response.id, params).await?;
///     println!("Found {} input items", input_items.data.len());
///
///     // Delete the response
///     client.delete_response(&response.id).await?;
///     println!("Deleted response");
///
///     Ok(())
/// }
/// ```
pub trait ResponsesService {
    /// Creates a new response.
    ///
    /// # Arguments
    ///
    /// * `request` - The response data to create
    ///
    /// # Returns
    ///
    /// Returns the created `Response` object with its assigned ID and metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be parsed.
    fn create_response(
        &self,
        request: CreateResponseRequest,
    ) -> impl Future<Output = Result<Response>>;

    /// Retrieves a specific response by ID.
    ///
    /// # Arguments
    ///
    /// * `response_id` - The unique identifier of the response to retrieve
    ///
    /// # Returns
    ///
    /// Returns the `Response` object if found.
    ///
    /// # Errors
    ///
    /// Returns an error if the response is not found or the API request fails.
    fn get_response(&self, response_id: &str) -> impl Future<Output = Result<Response>>;

    /// Deletes a response.
    ///
    /// # Arguments
    ///
    /// * `response_id` - The unique identifier of the response to delete
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the response was successfully deleted.
    ///
    /// # Errors
    ///
    /// Returns an error if the response is not found or the deletion fails.
    fn delete_response(&self, response_id: &str) -> impl Future<Output = Result<()>>;

    /// Lists input items for a specific response.
    ///
    /// # Arguments
    ///
    /// * `response_id` - The unique identifier of the response
    /// * `params` - Pagination parameters (limit and offset)
    ///
    /// # Returns
    ///
    /// Returns a `ListInputItemsResponse` containing the input items and pagination info.
    ///
    /// # Errors
    ///
    /// Returns an error if the response is not found or the API request fails.
    fn list_input_items(
        &self,
        response_id: &str,
        params: ListInputItemsParams,
    ) -> impl Future<Output = Result<ListInputItemsResponse>>;
}

impl ResponsesService for PortkeyClient {
    async fn create_response(&self, request: CreateResponseRequest) -> Result<Response> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            trace_id = ?request.trace_id,
            model = ?request.model,
            "Creating response"
        );

        let response = self
            .send_json(reqwest::Method::POST, "/responses", &request)
            .await?;
        let response = response.error_for_status()?;
        let response_data: Response = response.json().await?;

        Ok(response_data)
    }

    async fn get_response(&self, response_id: &str) -> Result<Response> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            response_id = %response_id,
            "Getting response"
        );

        let path = format!("/responses/{}", response_id);
        let response = self.send(reqwest::Method::GET, &path).await?;
        let response = response.error_for_status()?;
        let response_data: Response = response.json().await?;

        Ok(response_data)
    }

    async fn delete_response(&self, response_id: &str) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            response_id = %response_id,
            "Deleting response"
        );

        let path = format!("/responses/{}", response_id);
        let response = self.send(reqwest::Method::DELETE, &path).await?;
        response.error_for_status()?;

        Ok(())
    }

    async fn list_input_items(
        &self,
        response_id: &str,
        params: ListInputItemsParams,
    ) -> Result<ListInputItemsResponse> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            response_id = %response_id,
            limit = ?params.limit,
            offset = ?params.offset,
            "Listing input items"
        );

        let path = format!("/responses/{}/input_items", response_id);
        let mut request = self.request_builder(reqwest::Method::GET, &path)?;

        // Add query parameters if specified
        if let Some(limit) = params.limit {
            request = request.query(&[("limit", limit.to_string())]);
        }
        if let Some(offset) = params.offset {
            request = request.query(&[("offset", offset.to_string())]);
        }

        let response = request.send().await?;
        let response = response.error_for_status()?;
        let input_items: ListInputItemsResponse = response.json().await?;

        Ok(input_items)
    }
}
