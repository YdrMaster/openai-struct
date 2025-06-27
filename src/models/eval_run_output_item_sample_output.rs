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
pub struct EvalRunOutputItemSampleOutput {
    /// The content of the message.
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// The role of the message (e.g. \"system\", \"assistant\", \"user\").
    #[serde(rename = "role")]
    pub role: Option<String>,
}
