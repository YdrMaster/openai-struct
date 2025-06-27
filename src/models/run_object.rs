/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunObject : Represents an execution run on a [thread](/docs/api-reference/threads).
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunObject {
    /// The ID of the [assistant](/docs/api-reference/assistants) used for execution of this run.
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    /// The Unix timestamp (in seconds) for when the run was cancelled.
    #[serde(rename = "cancelled_at")]
    pub cancelled_at: i32,
    /// The Unix timestamp (in seconds) for when the run was completed.
    #[serde(rename = "completed_at")]
    pub completed_at: i32,
    /// The Unix timestamp (in seconds) for when the run was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The Unix timestamp (in seconds) for when the run will expire.
    #[serde(rename = "expires_at")]
    pub expires_at: i32,
    /// The Unix timestamp (in seconds) for when the run failed.
    #[serde(rename = "failed_at")]
    pub failed_at: i32,
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "incomplete_details")]
    pub incomplete_details: crate::models::RunObjectIncompleteDetails,
    /// The instructions that the [assistant](/docs/api-reference/assistants) used for this run.
    #[serde(rename = "instructions")]
    pub instructions: String,
    #[serde(rename = "last_error")]
    pub last_error: crate::models::RunObjectLastError,
    /// The maximum number of completion tokens specified to have been used over the course of the run.
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: i32,
    /// The maximum number of prompt tokens specified to have been used over the course of the run.
    #[serde(rename = "max_prompt_tokens")]
    pub max_prompt_tokens: i32,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// The model that the [assistant](/docs/api-reference/assistants) used for this run.
    #[serde(rename = "model")]
    pub model: String,
    /// The object type, which is always `thread.run`.
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: crate::models::ParallelToolCalls,
    #[serde(rename = "required_action")]
    pub required_action: crate::models::RunObjectRequiredAction,
    #[serde(rename = "response_format")]
    pub response_format: crate::models::AssistantsApiResponseFormatOption,
    /// The Unix timestamp (in seconds) for when the run was started.
    #[serde(rename = "started_at")]
    pub started_at: i32,
    /// The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`.
    #[serde(rename = "status")]
    pub status: String,
    /// The sampling temperature used for this run. If not set, defaults to 1.
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
    /// The ID of the [thread](/docs/api-reference/threads) that was executed on as a part of this run.
    #[serde(rename = "thread_id")]
    pub thread_id: String,
    #[serde(rename = "tool_choice")]
    pub tool_choice: crate::models::CreateRunRequestToolChoice,
    /// The list of tools that the [assistant](/docs/api-reference/assistants) used for this run.
    #[serde(rename = "tools")]
    pub tools: Vec<Value>,
    /// The nucleus sampling value used for this run. If not set, defaults to 1.
    #[serde(rename = "top_p")]
    pub top_p: Option<f32>,
    #[serde(rename = "truncation_strategy")]
    pub truncation_strategy: crate::models::CreateRunRequestToolChoice,
    #[serde(rename = "usage")]
    pub usage: crate::models::RunCompletionUsage,
}
