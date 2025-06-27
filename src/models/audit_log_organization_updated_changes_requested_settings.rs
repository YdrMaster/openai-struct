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
pub struct AuditLogOrganizationUpdatedChangesRequestedSettings {
    /// Visibility of the threads page which shows messages created with the Assistants API and Playground. One of `ANY_ROLE`, `OWNERS`, or `NONE`.
    #[serde(rename = "threads_ui_visibility")]
    pub threads_ui_visibility: Option<String>,
    /// Visibility of the usage dashboard which shows activity and costs for your organization. One of `ANY_ROLE` or `OWNERS`.
    #[serde(rename = "usage_dashboard_visibility")]
    pub usage_dashboard_visibility: Option<String>,
}
