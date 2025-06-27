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
pub struct CompletionUsage {
    /// Number of tokens in the generated completion.
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i32,
    #[serde(rename = "completion_tokens_details")]
    pub completion_tokens_details: Option<crate::models::CompletionUsageCompletionTokensDetails>,
    /// Number of tokens in the prompt.
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i32,
    #[serde(rename = "prompt_tokens_details")]
    pub prompt_tokens_details: Option<crate::models::CompletionUsagePromptTokensDetails>,
    /// Total number of tokens used in the request (prompt + completion).
    #[serde(rename = "total_tokens")]
    pub total_tokens: i32,
}
