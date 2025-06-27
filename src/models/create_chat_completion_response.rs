/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CreateChatCompletionResponse : Represents a chat completion response returned by model, based on the provided input.
// todo: 过长之后看
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateChatCompletionResponse {
    /// A list of chat completion choices. Can be more than one if `n` is greater than 1.
    #[serde(rename = "choices")]
    pub choices: Vec<crate::models::CreateChatCompletionResponseChoices>,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    #[serde(rename = "created")]
    pub created: i32,
    /// A unique identifier for the chat completion.
    #[serde(rename = "id")]
    pub id: String,
    /// The model used for the chat completion.
    #[serde(rename = "model")]
    pub model: String,
    /// The object type, which is always `chat.completion`.
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: String,
    #[serde(rename = "service_tier")]
    pub service_tier: Option<crate::models::ServiceTier>,
    /// This fingerprint represents the backend configuration that the model runs with.  Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: Option<String>,
    #[serde(rename = "usage")]
    pub usage: Option<crate::models::CompletionUsage>,
}

fn default_object() -> String {
    "chat.completion".into()
}
