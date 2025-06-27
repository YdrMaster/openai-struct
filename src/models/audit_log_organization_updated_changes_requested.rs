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
pub struct AuditLogOrganizationUpdatedChangesRequested {
    /// The organization description.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The organization name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "settings")]
    pub settings: Option<crate::models::AuditLogOrganizationUpdatedChangesRequestedSettings>,
    /// The organization title.
    #[serde(rename = "title")]
    pub title: Option<String>,
}
