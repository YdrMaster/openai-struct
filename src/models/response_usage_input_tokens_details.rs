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
pub struct ResponseUsageInputTokensDetails {
    /// The number of tokens that were retrieved from the cache.  [More on prompt caching](/docs/guides/prompt-caching).
    #[serde(rename = "cached_tokens")]
    pub cached_tokens: i32,
}
