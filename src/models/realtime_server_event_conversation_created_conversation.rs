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
pub struct RealtimeServerEventConversationCreatedConversation {
    /// The unique ID of the conversation.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The object type, must be `realtime.conversation`.
    #[serde(rename = "object")]
    pub object: Option<String>,
}
