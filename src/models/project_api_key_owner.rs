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
pub struct ProjectApiKeyOwner {
    #[serde(rename = "service_account")]
    pub service_account: Option<crate::models::ProjectServiceAccount>,
    /// `user` or `service_account`
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(rename = "user")]
    pub user: Option<crate::models::ProjectUser>,
}
