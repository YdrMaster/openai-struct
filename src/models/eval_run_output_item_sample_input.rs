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
pub struct EvalRunOutputItemSampleInput {
    /// The content of the message.
    #[serde(rename = "content")]
    pub content: String,
    /// The role of the message sender (e.g., system, user, developer).
    #[serde(rename = "role")]
    pub role: String,
}
