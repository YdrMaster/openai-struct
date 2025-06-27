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
pub struct ThreadObject {
    /// The Unix timestamp (in seconds) for when the thread was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// The object type, which is always `thread`.
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: String,
    #[serde(rename = "tool_resources")]
    pub tool_resources: crate::models::ModifyThreadRequestToolResources,
}

fn default_object() -> String {
    return "thread".into();
}
