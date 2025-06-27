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
/// AuditLogEventType:
///   type: string
///   description: The event type.
///   enum:
///     - api_key.created
///     - api_key.updated
///     - api_key.deleted
///     - checkpoint_permission.created
///     - checkpoint_permission.deleted
///     - invite.sent
///     - invite.accepted
///     - invite.deleted
///     - login.succeeded
///     - login.failed
///     - logout.succeeded
///     - logout.failed
///     - organization.updated
///     - project.created
///     - project.updated
///     - project.archived
///     - service_account.created
///     - service_account.updated
///     - service_account.deleted
///     - rate_limit.updated
///     - rate_limit.deleted
///     - user.added
///     - user.updated
///     - user.deleted
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AuditLogEventType {
    #[serde(rename = "api_key.created")]
    ApiKeyCreated,
    #[serde(rename = "api_key.updated")]
    ApiKeyUpdated,
    #[serde(rename = "api_key.deleted")]
    ApiKeyDeleted,
    #[serde(rename = "checkpoint_permission.created")]
    CheckpointPermissionCreated,
    #[serde(rename = "checkpoint_permission.deleted")]
    CheckpointPermissionDeleted,
    #[serde(rename = "invite.sent")]
    InviteSent,
    #[serde(rename = "invite.accepted")]
    InviteAccepted,
    #[serde(rename = "invite.deleted")]
    InviteDeleted,
    #[serde(rename = "login.succeeded")]
    LoginSucceeded,
    #[serde(rename = "login.failed")]
    LoginFailed,
    #[serde(rename = "logout.succeeded")]
    LogoutSucceeded,
    #[serde(rename = "logout.failed")]
    LogoutFailed,
    #[serde(rename = "organization.updated")]
    OrganizationUpdated,
    #[serde(rename = "project.created")]
    ProjectCreated,
    #[serde(rename = "project.updated")]
    ProjectUpdated,
    #[serde(rename = "project.archived")]
    ProjectArchived,
    #[serde(rename = "service_account.created")]
    ServiceAccountCreated,
    #[serde(rename = "service_account.updated")]
    ServiceAccountUpdated,
    #[serde(rename = "service_account.deleted")]
    ServiceAccountDeleted,
    #[serde(rename = "rate_limit.updated")]
    RateLimitUpdated,
    #[serde(rename = "rate_limit.deleted")]
    RateLimitDeleted,
    #[serde(rename = "user.added")]
    UserAdded,
    #[serde(rename = "user.updated")]
    UserUpdated,
    #[serde(rename = "user.deleted")]
    UserDeleted,
}
