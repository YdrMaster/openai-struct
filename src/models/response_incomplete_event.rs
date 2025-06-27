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
pub struct ResponseIncompleteEvent {
    /// The response that was incomplete.
    #[serde(rename = "response")]
    pub response: crate::models::Response,
    /// The type of the event. Always `response.incomplete`.
    #[serde(rename = "type")]
    pub _type: String,
}
