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
pub struct CreateCompletionResponseLogprobs {
    #[serde(rename = "text_offset")]
    pub text_offset: Option<Vec<i32>>,
    #[serde(rename = "token_logprobs")]
    pub token_logprobs: Option<Vec<f32>>,
    #[serde(rename = "tokens")]
    pub tokens: Option<Vec<String>>,
    #[serde(rename = "top_logprobs")]
    pub top_logprobs: Option<Vec<::std::collections::HashMap<String, f32>>>,
}
