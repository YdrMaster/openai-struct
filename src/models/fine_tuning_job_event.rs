/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub FineTuningJobEvent : Fine-tuning job event object
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FineTuningJobEvent {
    /// The Unix timestamp (in seconds) for when the fine-tuning job was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The data associated with the event.
    #[serde(rename = "data")]
    pub data: Option<Value>,
    /// The object identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// The log level of the event.
    #[serde(rename = "level")]
    pub level: String,
    /// The message of the event.
    #[serde(rename = "message")]
    pub message: String,
    /// The object type, which is always \"fine_tuning.job.event\".
    #[serde(rename = "object")]
    pub object: String,
    /// The type of event.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
