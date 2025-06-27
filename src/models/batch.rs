/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// .
#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    /// The Unix timestamp (in seconds) for when the batch was cancelled.
    #[serde(rename = "cancelled_at")]
    pub cancelled_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch started cancelling.
    #[serde(rename = "cancelling_at")]
    pub cancelling_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch was completed.
    #[serde(rename = "completed_at")]
    pub completed_at: Option<i32>,
    /// The time frame within which the batch should be processed.
    #[serde(rename = "completion_window")]
    pub completion_window: String,
    /// The Unix timestamp (in seconds) for when the batch was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The OpenAI API endpoint used by the batch.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// The ID of the file containing the outputs of requests with errors.
    #[serde(rename = "error_file_id")]
    pub error_file_id: Option<String>,
    #[serde(rename = "errors")]
    pub errors: Option<crate::models::BatchErrors>,
    /// The Unix timestamp (in seconds) for when the batch expired.
    #[serde(rename = "expired_at")]
    pub expired_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch will expire.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch failed.
    #[serde(rename = "failed_at")]
    pub failed_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch started finalizing.
    #[serde(rename = "finalizing_at")]
    pub finalizing_at: Option<i32>,
    #[serde(rename = "id")]
    pub id: String,
    /// The Unix timestamp (in seconds) for when the batch started processing.
    #[serde(rename = "in_progress_at")]
    pub in_progress_at: Option<i32>,
    /// The ID of the input file for the batch.
    #[serde(rename = "input_file_id")]
    pub input_file_id: String,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    /// The object type, which is always `batch`.
    #[serde(rename = "object")]
    pub object: String,
    /// The ID of the file containing the outputs of successfully executed requests.
    #[serde(rename = "output_file_id")]
    pub output_file_id: Option<String>,
    #[serde(rename = "request_counts")]
    pub request_counts: Option<crate::models::BatchRequestCounts>,
    /// The current status of the batch.
    #[serde(rename = "status")]
    pub status: String,
}
