use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Request to create a fine-tuning job.
///
/// # Example
///
/// ```rust,ignore
/// use portkey::model::CreateFineTuningJobRequest;
///
/// let request = CreateFineTuningJobRequest::builder()
///     .model("gpt-3.5-turbo")
///     .training_file("file-abc123")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CreateFineTuningJobRequest {
    /// The name of the model to fine-tune.
    pub model: String,

    /// The ID of an uploaded file that contains training data.
    pub training_file: String,

    /// The hyperparameters used for the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparameters: Option<Hyperparameters>,

    /// A string of up to 18 characters that will be added to your fine-tuned model name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// The ID of an uploaded file that contains validation data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,

    /// A list of integrations to enable for your fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<Integration>>,

    /// The seed controls the reproducibility of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

/// The hyperparameters used for the fine-tuning job.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Hyperparameters {
    /// Number of examples in each batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<HyperparameterValue>,

    /// Scaling factor for the learning rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<HyperparameterValue>,

    /// The number of epochs to train the model for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_epochs: Option<HyperparameterValue>,
}

/// A hyperparameter value can be either "auto" or a specific number.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperparameterValue {
    Auto(String),
    Number(f64),
}

/// Integration configuration for a fine-tuning job.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Integration {
    /// The type of integration to enable.
    #[serde(rename = "type")]
    pub integration_type: String,

    /// The settings for the wandb integration.
    pub wandb: WandbIntegration,
}

/// Weights & Biases integration configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WandbIntegration {
    /// The name of the project that the new run will be created under.
    pub project: String,

    /// A display name to set for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The entity to use for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// A list of tags to be attached to the newly created run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// The fine-tuning job object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FineTuningJob {
    /// The object identifier, which can be referenced in the API endpoints.
    pub id: String,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was created.
    pub created_at: i64,

    /// For fine-tuning jobs that have failed, this will contain more information on the cause of the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<FineTuningError>,

    /// The name of the fine-tuned model that is being created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuned_model: Option<String>,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,

    /// The hyperparameters used for the fine-tuning job.
    pub hyperparameters: Hyperparameters,

    /// The base model that is being fine-tuned.
    pub model: String,

    /// The object type, which is always "fine_tuning.job".
    pub object: String,

    /// The organization that owns the fine-tuning job.
    pub organization_id: String,

    /// The compiled results file ID(s) for the fine-tuning job.
    pub result_files: Vec<String>,

    /// The current status of the fine-tuning job.
    pub status: String,

    /// The total number of billable tokens processed by this fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trained_tokens: Option<i64>,

    /// The file ID used for training.
    pub training_file: String,

    /// The file ID used for validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,

    /// A list of integrations to enable for your fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<Integration>>,

    /// The seed used for the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// The Unix timestamp (in seconds) for when the fine-tuning job is estimated to finish.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_finish: Option<i64>,
}

/// Error information for a failed fine-tuning job.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FineTuningError {
    /// A machine-readable error code.
    pub code: String,

    /// A human-readable error message.
    pub message: String,

    /// The parameter that was invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

/// Response containing a list of fine-tuning jobs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListFineTuningJobsResponse {
    pub object: String,
    pub data: Vec<FineTuningJob>,
    pub has_more: bool,
}

/// A fine-tuning job event object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FineTuningJobEvent {
    pub id: String,
    pub created_at: i64,
    pub level: String,
    pub message: String,
    pub object: String,
}

/// Response containing a list of fine-tuning job events.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListFineTuningJobEventsResponse {
    pub object: String,
    pub data: Vec<FineTuningJobEvent>,
}

/// A fine-tuning job checkpoint object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FineTuningJobCheckpoint {
    /// The checkpoint identifier, which can be referenced in the API endpoints.
    pub id: String,

    /// The Unix timestamp (in seconds) for when the checkpoint was created.
    pub created_at: i64,

    /// The name of the fine-tuned checkpoint model that is created.
    pub fine_tuned_model_checkpoint: String,

    /// The step number that the checkpoint was created at.
    pub step_number: i64,

    /// Metrics at the step number during the fine-tuning job.
    pub metrics: HashMap<String, f64>,

    /// The name of the fine-tuning job that this checkpoint was created from.
    pub fine_tuning_job_id: String,

    /// The object type, which is always "fine_tuning.job.checkpoint".
    pub object: String,
}

/// Response containing a list of fine-tuning job checkpoints.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListFineTuningJobCheckpointsResponse {
    pub object: String,
    pub data: Vec<FineTuningJobCheckpoint>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}
