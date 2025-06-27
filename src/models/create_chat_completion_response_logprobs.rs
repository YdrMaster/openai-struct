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
pub struct CreateChatCompletionResponseLogprobs {
    /// A list of message content tokens with log probability information.
    #[serde(rename = "content")]
    pub content: Vec<crate::models::ChatCompletionTokenLogprob>,
    /// A list of message refusal tokens with log probability information.
    #[serde(rename = "refusal")]
    pub refusal: Vec<crate::models::ChatCompletionTokenLogprob>,
}
