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
pub struct RealtimeClientEventResponseCreate {
    /// Optional client-generated ID used to identify this event.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[serde(rename = "response")]
    pub response: Option<crate::models::RealtimeResponseCreateParams>,
    /// The event type, must be `response.create`.
    #[serde(rename = "type")]
    pub _type: String,
}
