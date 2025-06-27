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
pub struct Invite {
    /// The Unix timestamp (in seconds) of when the invite was accepted.
    #[serde(rename = "accepted_at")]
    pub accepted_at: Option<i32>,
    /// The email address of the individual to whom the invite was sent
    #[serde(rename = "email")]
    pub email: String,
    /// The Unix timestamp (in seconds) of when the invite expires.
    #[serde(rename = "expires_at")]
    pub expires_at: i32,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: String,
    /// The Unix timestamp (in seconds) of when the invite was sent.
    #[serde(rename = "invited_at")]
    pub invited_at: i32,
    /// The object type, which is always `organization.invite`
    #[serde(rename = "object")]
    pub object: String,
    /// The projects that were granted membership upon acceptance of the invite.
    #[serde(rename = "projects")]
    pub projects: Option<Vec<crate::models::InviteProjects>>,
    /// `owner` or `reader`
    #[serde(rename = "role")]
    pub role: String,
    /// `accepted`,`expired`, or `pending`
    #[serde(rename = "status")]
    pub status: String,
}
