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
pub struct ProjectServiceAccount {
    /// The Unix timestamp (in seconds) of when the service account was created
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the service account
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `organization.project.service_account`
    #[serde(rename = "object")]
    pub object: String,
    /// `owner` or `member`
    #[serde(rename = "role")]
    pub role: String,
}
