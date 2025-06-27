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
pub struct ChatCompletionDeleted {
    /// Whether the chat completion was deleted.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// The ID of the chat completion that was deleted.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of object being deleted. enum: - chat.completion.deleted.
    #[serde(rename = "object")]
    pub object: String,
}
