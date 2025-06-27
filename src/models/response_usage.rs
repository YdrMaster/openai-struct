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
pub struct ResponseUsage {
    /// The number of input tokens.
    #[serde(rename = "input_tokens")]
    pub input_tokens: i32,
    #[serde(rename = "input_tokens_details")]
    pub input_tokens_details: crate::models::ResponseUsageInputTokensDetails,
    /// The number of output tokens.
    #[serde(rename = "output_tokens")]
    pub output_tokens: i32,
    #[serde(rename = "output_tokens_details")]
    pub output_tokens_details: crate::models::ResponseUsageOutputTokensDetails,
    /// The total number of tokens used.
    #[serde(rename = "total_tokens")]
    pub total_tokens: i32,
}
