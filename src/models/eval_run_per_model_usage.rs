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
pub struct EvalRunPerModelUsage {
    /// The number of tokens retrieved from cache.
    #[serde(rename = "cached_tokens")]
    pub cached_tokens: i32,
    /// The number of completion tokens generated.
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i32,
    /// The number of invocations.
    #[serde(rename = "invocation_count")]
    pub invocation_count: i32,
    /// The name of the model.
    #[serde(rename = "model_name")]
    pub model_name: String,
    /// The number of prompt tokens used.
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i32,
    /// The total number of tokens used.
    #[serde(rename = "total_tokens")]
    pub total_tokens: i32,
}
