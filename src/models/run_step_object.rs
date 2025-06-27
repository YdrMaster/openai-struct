/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunStepObject : Represents a step in execution of a run.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunStepObject {
    /// The ID of the [assistant](/docs/api-reference/assistants) associated with the run step.
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    /// The Unix timestamp (in seconds) for when the run step was cancelled.
    #[serde(rename = "cancelled_at")]
    pub cancelled_at: i32,
    /// The Unix timestamp (in seconds) for when the run step completed.
    #[serde(rename = "completed_at")]
    pub completed_at: i32,
    /// The Unix timestamp (in seconds) for when the run step was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The Unix timestamp (in seconds) for when the run step expired. A step is considered expired if the parent run is expired.
    #[serde(rename = "expired_at")]
    pub expired_at: i32,
    /// The Unix timestamp (in seconds) for when the run step failed.
    #[serde(rename = "failed_at")]
    pub failed_at: i32,
    /// The identifier of the run step, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_error")]
    pub last_error: crate::models::RunStepObjectLastError,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// The object type, which is always `thread.run.step`.
    #[serde(rename = "object")]
    pub object: String,
    /// The ID of the [run](/docs/api-reference/runs) that this run step is a part of.
    #[serde(rename = "run_id")]
    pub run_id: String,
    /// The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`.
    #[serde(rename = "status")]
    pub status: String,
    /// The details of the run step.
    #[serde(rename = "step_details")]
    pub step_details: Value,
    /// The ID of the [thread](/docs/api-reference/threads) that was run.
    #[serde(rename = "thread_id")]
    pub thread_id: String,
    /// The type of run step, which can be either `message_creation` or `tool_calls`.
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "usage")]
    pub usage: crate::models::RunStepCompletionUsage,
}
