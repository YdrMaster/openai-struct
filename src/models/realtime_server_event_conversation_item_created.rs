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
pub struct RealtimeServerEventConversationItemCreated {
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "item")]
    pub item: crate::models::RealtimeConversationItem,
    /// The ID of the preceding item in the Conversation context, allows the  client to understand the order of the conversation.
    #[serde(rename = "previous_item_id")]
    pub previous_item_id: String,
    /// The event type, must be `conversation.item.created`.
    #[serde(rename = "type")]
    pub _type: String,
}
