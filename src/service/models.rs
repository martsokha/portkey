//! Models API service.
//!
//! This module provides methods for listing available models through Portkey.

use std::future::Future;

use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{ListModelsParams, ListModelsResponse, ModelSortField, SortOrder};

/// Trait for Models API operations.
pub trait ModelsService {
    /// Lists the currently available models.
    ///
    /// Returns information about models that can be used through Portkey.
    /// Supports filtering by provider, AI service, and pagination.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a `ListModelsResponse` containing available models.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use portkey_sdk::{PortkeyConfig, PortkeyClient};
    /// use portkey_sdk::service::ModelsService;
    /// use portkey_sdk::model::ListModelsParams;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build()?;
    /// let client = PortkeyClient::new(config)?;
    ///
    /// // List all models
    /// let models = client.list_models(None).await?;
    /// println!("Found {} models", models.data.len());
    ///
    /// // List models with filters
    /// let params = ListModelsParams {
    ///     provider: Some("openai".to_string()),
    ///     limit: Some(10),
    ///     ..Default::default()
    /// };
    /// let models = client.list_models(Some(params)).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn list_models(
        &self,
        params: Option<ListModelsParams>,
    ) -> impl Future<Output = Result<ListModelsResponse>>;
}

impl ModelsService for PortkeyClient {
    async fn list_models(&self, params: Option<ListModelsParams>) -> Result<ListModelsResponse> {
        let mut request = self.request_builder(reqwest::Method::GET, "/models")?;

        // Add query parameters if provided
        if let Some(p) = params {
            if let Some(ai_service) = p.ai_service {
                request = request.query(&[("ai_service", ai_service)]);
            }
            if let Some(provider) = p.provider {
                request = request.query(&[("provider", provider)]);
            }
            if let Some(limit) = p.limit {
                request = request.query(&[("limit", limit.to_string())]);
            }
            if let Some(offset) = p.offset {
                request = request.query(&[("offset", offset.to_string())]);
            }
            if let Some(sort) = p.sort {
                let sort_str = match sort {
                    ModelSortField::Name => "name",
                    ModelSortField::Provider => "provider",
                    ModelSortField::AiService => "ai_service",
                };
                request = request.query(&[("sort", sort_str)]);
            }
            if let Some(order) = p.order {
                let order_str = match order {
                    SortOrder::Asc => "asc",
                    SortOrder::Desc => "desc",
                };
                request = request.query(&[("order", order_str)]);
            }
        }

        let response = request.send().await?;
        let response = response.error_for_status()?;
        let models_response: ListModelsResponse = response.json().await?;
        Ok(models_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_models_params() {
        let params = ListModelsParams {
            provider: Some("openai".to_string()),
            limit: Some(10),
            ..Default::default()
        };

        assert_eq!(params.provider, Some("openai".to_string()));
        assert_eq!(params.limit, Some(10));
    }
}
