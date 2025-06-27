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
pub struct CreateChatCompletionStreamResponse {
    /// A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the last chunk if you set `stream_options: {\"include_usage\": true}`.
    #[serde(rename = "choices")]
    pub choices: Vec<crate::models::CreateChatCompletionStreamResponseChoices>,
    /// The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp.
    #[serde(rename = "created")]
    pub created: i32,
    /// A unique identifier for the chat completion. Each chunk has the same ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The model to generate the completion.
    #[serde(rename = "model")]
    pub model: String,
    /// The object type, which is always `chat.completion.chunk`.
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "service_tier")]
    pub service_tier: Option<crate::models::ServiceTier>,
    /// This fingerprint represents the backend configuration that the model runs with. Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: Option<String>,
    /// An optional field that will only be present when you set `stream_options: {\"include_usage\": true}` in your request. When present, it contains a null value **except for the last chunk** which contains the token usage statistics for the entire request.  **NOTE:** If the stream is interrupted or cancelled, you may not receive the final usage chunk which contains the total token usage for the request.
    #[serde(rename = "usage")]
    pub usage: Option<crate::models::CompletionUsage>,
}

impl Default for CreateChatCompletionStreamResponse {
    fn default() -> CreateChatCompletionStreamResponse {
        Self {
            choices: vec![],
            created: 0,
            id: "".to_string(),
            model: "".to_string(),
            object: "".to_string(),
            service_tier: None,
            system_fingerprint: None,
            usage: None,
        }
    }
}
