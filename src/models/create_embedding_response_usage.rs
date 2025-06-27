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
pub struct CreateEmbeddingResponseUsage {
    /// The number of tokens used by the prompt.
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i32,
    /// The total number of tokens used by the request.
    #[serde(rename = "total_tokens")]
    pub total_tokens: i32,
}
