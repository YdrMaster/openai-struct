/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub FineTuningJob : The `fine_tuning.job` object represents a fine-tuning job that has been created through the API.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FineTuningJob {
    /// The Unix timestamp (in seconds) for when the fine-tuning job was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "error")]
    pub error: crate::models::FineTuningJobError,
    /// The Unix timestamp (in seconds) for when the fine-tuning job is estimated to finish. The value will be null if the fine-tuning job is not running.
    #[serde(rename = "estimated_finish")]
    pub estimated_finish: Option<i32>,
    /// The name of the fine-tuned model that is being created. The value will be null if the fine-tuning job is still running.
    #[serde(rename = "fine_tuned_model")]
    pub fine_tuned_model: String,
    /// The Unix timestamp (in seconds) for when the fine-tuning job was finished. The value will be null if the fine-tuning job is still running.
    #[serde(rename = "finished_at")]
    pub finished_at: i32,
    #[serde(rename = "hyperparameters")]
    pub hyperparameters: crate::models::FineTuningJobHyperparameters,
    /// The object identifier, which can be referenced in the API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of integrations to enable for this fine-tuning job.
    #[serde(rename = "integrations")]
    pub integrations: Option<Vec<Value>>,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    #[serde(rename = "method")]
    pub method: Option<crate::models::FineTuneMethod>,
    /// The base model that is being fine-tuned.
    #[serde(rename = "model")]
    pub model: String,
    /// The object type, which is always \"fine_tuning.job\".
    #[serde(rename = "object")]
    pub object: String,
    /// The organization that owns the fine-tuning job.
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    /// The compiled results file ID(s) for the fine-tuning job. You can retrieve the results with the [Files API](/docs/api-reference/files/retrieve-contents).
    #[serde(rename = "result_files")]
    pub result_files: Vec<String>,
    /// The seed used for the fine-tuning job.
    #[serde(rename = "seed")]
    pub seed: i32,
    /// The current status of the fine-tuning job, which can be either `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`.
    #[serde(rename = "status")]
    pub status: String,
    /// The total number of billable tokens processed by this fine-tuning job. The value will be null if the fine-tuning job is still running.
    #[serde(rename = "trained_tokens")]
    pub trained_tokens: i32,
    /// The file ID used for training. You can retrieve the training data with the [Files API](/docs/api-reference/files/retrieve-contents).
    #[serde(rename = "training_file")]
    pub training_file: String,
    /// The file ID used for validation. You can retrieve the validation results with the [Files API](/docs/api-reference/files/retrieve-contents).
    #[serde(rename = "validation_file")]
    pub validation_file: String,
}
