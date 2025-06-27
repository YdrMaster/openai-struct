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
pub struct OutputMessage {
    /// The content of the output message.
    #[serde(rename = "content")]
    pub content: Vec<crate::models::OutputContent>,
    /// The unique ID of the output message.
    #[serde(rename = "id")]
    pub id: String,
    /// The role of the output message. Always `assistant`.
    #[serde(rename = "role")]
    pub role: String,
    /// The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API.
    #[serde(rename = "status")]
    pub status: String,
    /// The type of the output message. Always `message`.
    #[serde(rename = "type")]
    pub _type: String,
}
