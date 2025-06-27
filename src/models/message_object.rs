/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub MessageObject : Represents a message within a [thread](/docs/api-reference/threads).
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageObject {
    /// If applicable, the ID of the [assistant](/docs/api-reference/assistants) that authored this message.
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    /// A list of files attached to the message, and the tools they were added to.
    #[serde(rename = "attachments")]
    pub attachments: Vec<crate::models::CreateMessageRequestAttachments>,
    /// The Unix timestamp (in seconds) for when the message was completed.
    #[serde(rename = "completed_at")]
    pub completed_at: i32,
    /// The content of the message in array of text and/or images.
    #[serde(rename = "content")]
    pub content: Vec<Value>,
    /// The Unix timestamp (in seconds) for when the message was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The Unix timestamp (in seconds) for when the message was marked as incomplete.
    #[serde(rename = "incomplete_at")]
    pub incomplete_at: i32,
    #[serde(rename = "incomplete_details")]
    pub incomplete_details: crate::models::MessageObjectIncompleteDetails,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// The object type, which is always `thread.message`.
    #[serde(rename = "object")]
    pub object: String,
    /// The entity that produced the message. One of `user` or `assistant`.
    #[serde(rename = "role")]
    pub role: String,
    /// The ID of the [run](/docs/api-reference/runs) associated with the creation of this message. Value is `null` when messages are created manually using the create message or create thread endpoints.
    #[serde(rename = "run_id")]
    pub run_id: String,
    /// The status of the message, which can be either `in_progress`, `incomplete`, or `completed`.
    #[serde(rename = "status")]
    pub status: String,
    /// The [thread](/docs/api-reference/threads) ID that this message belongs to.
    #[serde(rename = "thread_id")]
    pub thread_id: String,
}
