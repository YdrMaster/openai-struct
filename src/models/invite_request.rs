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
pub struct InviteRequest {
    /// Send an email to this address
    #[serde(rename = "email")]
    pub email: String,
    /// An array of projects to which membership is granted at the same time the org invite is accepted. If omitted, the user will be invited to the default project for compatibility with legacy behavior.
    #[serde(rename = "projects")]
    pub projects: Option<Vec<crate::models::InviteRequestProjects>>,
    /// `owner` or `reader`
    #[serde(rename = "role")]
    pub role: String,
}
