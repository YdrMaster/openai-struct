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
pub struct ProjectApiKey {
    /// The Unix timestamp (in seconds) of when the API key was created
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: String,
    /// The Unix timestamp (in seconds) of when the API key was last used.
    #[serde(rename = "last_used_at")]
    pub last_used_at: i32,
    /// The name of the API key
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `organization.project.api_key`
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "owner")]
    pub owner: crate::models::ProjectApiKeyOwner,
    /// The redacted value of the API key
    #[serde(rename = "redacted_value")]
    pub redacted_value: String,
}
