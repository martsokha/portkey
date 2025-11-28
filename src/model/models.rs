//! Models API data structures.
//!
//! This module contains data models for listing available models through Portkey.

use serde::{Deserialize, Serialize};

/// Sort field for models.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ModelSortField {
    /// Sort by model name
    #[default]
    Name,
    /// Sort by provider
    Provider,
    /// Sort by AI service
    #[serde(rename = "ai_service")]
    AiService,
}

/// Sort order for models.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending order
    #[default]
    Asc,
    /// Descending order
    Desc,
}

/// Parameters for listing models.
///
/// # Example
///
/// ```
/// use portkey_sdk::model::{ListModelsParams, ModelSortField, SortOrder};
///
/// let params = ListModelsParams {
///     ai_service: Some("openai".to_string()),
///     provider: Some("openai".to_string()),
///     limit: Some(10),
///     offset: Some(0),
///     sort: Some(ModelSortField::Name),
///     order: Some(SortOrder::Asc),
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListModelsParams {
    /// Filter models by the AI service (e.g., 'openai', 'anthropic').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_service: Option<String>,

    /// Filter models by the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// The maximum number of models to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// The number of models to skip before starting to collect the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// The field to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ModelSortField>,

    /// The order to sort the results in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SortOrder>,
}

/// Describes a model offering that can be used with the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// The model identifier, which can be referenced in the API endpoints.
    pub id: String,

    /// The Unix timestamp (in seconds) when the model was created.
    pub created: i64,

    /// The object type, which is always "model".
    pub object: String,

    /// The organization that owns the model.
    pub owned_by: String,
}

/// Response from listing models.
///
/// # Example
///
/// ```
/// use portkey_sdk::model::ListModelsResponse;
///
/// // This would typically be deserialized from the API response
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelsResponse {
    /// The object type, which is always "list".
    pub object: String,

    /// Array of model objects.
    pub data: Vec<Model>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_sort_field() {
        let field = ModelSortField::Name;
        let json = serde_json::to_string(&field).unwrap();
        assert_eq!(json, "\"name\"");

        let field = ModelSortField::AiService;
        let json = serde_json::to_string(&field).unwrap();
        assert_eq!(json, "\"ai_service\"");
    }

    #[test]
    fn test_sort_order() {
        let order = SortOrder::Asc;
        let json = serde_json::to_string(&order).unwrap();
        assert_eq!(json, "\"asc\"");

        let order = SortOrder::Desc;
        let json = serde_json::to_string(&order).unwrap();
        assert_eq!(json, "\"desc\"");
    }

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
