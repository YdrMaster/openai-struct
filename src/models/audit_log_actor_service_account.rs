/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// AuditLogActorServiceAccount:
///   type: object
///   description: The service account that performed the audit logged action.
///   properties:
///     id:
///       type: string
///       description: The service account id.
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActorServiceAccount {
    /// The service account id.
    #[serde(rename = "id")]
    pub id: Option<String>,
}
