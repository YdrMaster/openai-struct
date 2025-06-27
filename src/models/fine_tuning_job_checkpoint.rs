/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct FineTuningJobCheckpoint {
    /// The Unix timestamp (in seconds) for when the checkpoint was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The name of the fine-tuned checkpoint model that is created.
    #[serde(rename = "fine_tuned_model_checkpoint")]
    pub fine_tuned_model_checkpoint: String,
    /// The name of the fine-tuning job that this checkpoint was created from.
    #[serde(rename = "fine_tuning_job_id")]
    pub fine_tuning_job_id: String,
    /// The checkpoint identifier, which can be referenced in the API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics")]
    pub metrics: crate::models::FineTuningJobCheckpointMetrics,
    /// The object type, which is always \"fine_tuning.job.checkpoint\".
    #[serde(rename = "object")]
    pub object: String,
    /// The step number that the checkpoint was created at.
    #[serde(rename = "step_number")]
    pub step_number: i32,
}
