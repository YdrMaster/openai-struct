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
pub struct RealtimeServerEventResponseOutputItemAdded {
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "item")]
    pub item: crate::models::RealtimeConversationItem,
    /// The index of the output item in the Response.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The ID of the Response to which the item belongs.
    #[serde(rename = "response_id")]
    pub response_id: String,
    /// The event type, must be `response.output_item.added`.
    #[serde(rename = "type")]
    pub _type: String,
}
