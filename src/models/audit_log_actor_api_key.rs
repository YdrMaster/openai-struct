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
/// AuditLogActorApiKey:
///   type: object
///   description: The API Key used to perform the audit logged action.
///   properties:
///     id:
///       type: string
///       description: The tracking id of the API key.
///     type:
///       type: string
///       description: The type of API key. Can be either `user` or `service_account`.
///       enum:
///         - user
///         - service_account
///     user:
///       $ref: "#/components/schemas/AuditLogActorUser"
///     service_account:
///       $ref: "#/components/schemas/AuditLogActorServiceAccount"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActorApiKey {
    /// The tracking id of the API key.
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "service_account")]
    pub service_account: Option<crate::models::AuditLogActorServiceAccount>,
    /// The type of API key. Can be either `user` or `service_account`.
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(rename = "user")]
    pub user: Option<crate::models::AuditLogActorUser>,
}
