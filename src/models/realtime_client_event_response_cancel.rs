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
pub struct RealtimeClientEventResponseCancel {
    /// Optional client-generated ID used to identify this event.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    /// A specific response ID to cancel - if not provided, will cancel an  in-progress response in the default conversation.
    #[serde(rename = "response_id")]
    pub response_id: Option<String>,
    /// The event type, must be `response.cancel`.
    #[serde(rename = "type")]
    pub _type: String,
}
