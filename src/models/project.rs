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
pub struct Project {
    /// The Unix timestamp (in seconds) of when the project was archived or `null`.
    #[serde(rename = "archived_at")]
    pub archived_at: Option<i32>,
    /// The Unix timestamp (in seconds) of when the project was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the project. This appears in reporting.
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `organization.project`
    #[serde(rename = "object")]
    pub object: String,
    /// `active` or `archived`
    #[serde(rename = "status")]
    pub status: String,
}
