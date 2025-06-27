/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateModerationRequest {
    /// Input (or inputs) to classify. Can be a single string, an array of strings, or an array of multi-modal input objects similar to other models.
    #[serde(rename = "input")]
    pub input: Value,
    /// The content moderation model you would like to use. Learn more in [the moderation guide](/docs/guides/moderation), and learn about available models [here](/docs/models#moderation).
    #[serde(rename = "model")]
    pub model: Option<Value>,
}
