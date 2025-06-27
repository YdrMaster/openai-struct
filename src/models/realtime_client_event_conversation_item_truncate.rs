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
pub struct RealtimeClientEventConversationItemTruncate {
    /// Inclusive duration up to which audio is truncated, in milliseconds. If  the audio_end_ms is greater than the actual audio duration, the server  will respond with an error.
    #[serde(rename = "audio_end_ms")]
    pub audio_end_ms: i32,
    /// The index of the content part to truncate. Set this to 0.
    #[serde(rename = "content_index")]
    pub content_index: i32,
    /// Optional client-generated ID used to identify this event.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    /// The ID of the assistant message item to truncate. Only assistant message  items can be truncated.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The event type, must be `conversation.item.truncate`.
    #[serde(rename = "type")]
    pub _type: String,
}
