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
pub struct AuditLogInviteSentData {
    /// The email invited to the organization.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The role the email was invited to be. Is either `owner` or `member`.
    #[serde(rename = "role")]
    pub role: Option<String>,
}
