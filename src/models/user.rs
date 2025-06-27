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
pub struct User {
    /// The Unix timestamp (in seconds) of when the user was added.
    #[serde(rename = "added_at")]
    pub added_at: i32,
    /// The email address of the user
    #[serde(rename = "email")]
    pub email: String,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the user
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `organization.user`
    #[serde(rename = "object")]
    pub object: String,
    /// `owner` or `reader`
    #[serde(rename = "role")]
    pub role: String,
}
