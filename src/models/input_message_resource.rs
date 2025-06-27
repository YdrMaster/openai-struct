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
pub struct InputMessageResource {
    #[serde(rename = "content")]
    pub content: crate::models::InputMessageContentList,
    /// The role of the message input. One of `user`, `system`, or `developer`.
    #[serde(rename = "role")]
    pub role: String,
    /// The status of item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The type of the message input. Always set to `message`.
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// The unique ID of the message input.
    #[serde(rename = "id")]
    pub id: String,
}
