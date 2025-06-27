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
pub struct AuditLogLoginFailed {
    /// The error code of the failure.
    #[serde(rename = "error_code")]
    pub error_code: Option<String>,
    /// The error message of the failure.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
}
