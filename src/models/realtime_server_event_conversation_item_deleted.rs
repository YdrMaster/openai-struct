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
pub struct RealtimeServerEventConversationItemDeleted {
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The ID of the item that was deleted.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The event type, must be `conversation.item.deleted`.
    #[serde(rename = "type")]
    pub _type: String,
}
