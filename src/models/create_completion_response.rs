/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// todo: 长，改天看
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompletionResponse {
    /// The list of completion choices the model generated for the input prompt.
    #[serde(rename = "choices")]
    pub choices: Vec<crate::models::CreateCompletionResponseChoices>,
    /// The Unix timestamp (in seconds) of when the completion was created.
    #[serde(rename = "created")]
    pub created: i32,
    /// A unique identifier for the completion.
    #[serde(rename = "id")]
    pub id: String,
    /// The model used for completion.
    #[serde(rename = "model")]
    pub model: String,
    /// The object type, which is always \"text_completion\"
    #[serde(rename = "object")]
    pub object: String,
    /// This fingerprint represents the backend configuration that the model runs with.  Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: Option<String>,
    #[serde(rename = "usage")]
    pub usage: Option<crate::models::CompletionUsage>,
}
