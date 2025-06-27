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
pub struct CreateCompletionResponseChoices {
    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence, `length` if the maximum number of tokens specified in the request was reached, or `content_filter` if content was omitted due to a flag from our content filters.
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "logprobs")]
    pub logprobs: crate::models::CreateCompletionResponseLogprobs,
    #[serde(rename = "text")]
    pub text: String,
}
