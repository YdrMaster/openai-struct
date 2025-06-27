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
pub struct AuditLog {
    #[serde(rename = "actor")]
    pub actor: crate::models::AuditLogActor,
    #[serde(rename = "api_key.created")]
    pub api_key_created: Option<crate::models::AuditLogApiKeyCreated>,
    #[serde(rename = "api_key.deleted")]
    pub api_key_deleted: Option<crate::models::AuditLogApiKeyDeleted>,
    #[serde(rename = "api_key.updated")]
    pub api_key_updated: Option<crate::models::AuditLogApiKeyUpdated>,
    #[serde(rename = "certificate.created")]
    pub certificate_created: Option<crate::models::AuditLogCertificateCreated>,
    #[serde(rename = "certificate.deleted")]
    pub certificate_deleted: Option<crate::models::AuditLogCertificateDeleted>,
    #[serde(rename = "certificate.updated")]
    pub certificate_updated: Option<crate::models::AuditLogCertificateCreated>,
    #[serde(rename = "certificates.activated")]
    pub certificates_activated: Option<crate::models::AuditLogCertificatesActivated>,
    #[serde(rename = "certificates.deactivated")]
    pub certificates_deactivated: Option<crate::models::AuditLogCertificatesActivated>,
    #[serde(rename = "checkpoint_permission.created")]
    pub checkpoint_permission_created: Option<crate::models::AuditLogCheckpointPermissionCreated>,
    #[serde(rename = "checkpoint_permission.deleted")]
    pub checkpoint_permission_deleted: Option<crate::models::AuditLogCheckpointPermissionDeleted>,
    /// The Unix timestamp (in seconds) of the event.
    #[serde(rename = "effective_at")]
    pub effective_at: i32,
    /// The ID of this log.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "invite.accepted")]
    pub invite_accepted: Option<crate::models::AuditLogInviteAccepted>,
    #[serde(rename = "invite.deleted")]
    pub invite_deleted: Option<crate::models::AuditLogInviteAccepted>,
    #[serde(rename = "invite.sent")]
    pub invite_sent: Option<crate::models::AuditLogInviteSent>,
    #[serde(rename = "login.failed")]
    pub login_failed: Option<crate::models::AuditLogLoginFailed>,
    #[serde(rename = "logout.failed")]
    pub logout_failed: Option<crate::models::AuditLogLoginFailed>,
    #[serde(rename = "organization.updated")]
    pub organization_updated: Option<crate::models::AuditLogOrganizationUpdated>,
    #[serde(rename = "project")]
    pub project: Option<crate::models::AuditLogProject>,
    #[serde(rename = "project.archived")]
    pub project_archived: Option<crate::models::AuditLogProjectArchived>,
    #[serde(rename = "project.created")]
    pub project_created: Option<crate::models::AuditLogProjectCreated>,
    #[serde(rename = "project.updated")]
    pub project_updated: Option<crate::models::AuditLogProjectUpdated>,
    #[serde(rename = "rate_limit.deleted")]
    pub rate_limit_deleted: Option<crate::models::AuditLogRateLimitDeleted>,
    #[serde(rename = "rate_limit.updated")]
    pub rate_limit_updated: Option<crate::models::AuditLogRateLimitUpdated>,
    #[serde(rename = "service_account.created")]
    pub service_account_created: Option<crate::models::AuditLogServiceAccountCreated>,
    #[serde(rename = "service_account.deleted")]
    pub service_account_deleted: Option<crate::models::AuditLogServiceAccountDeleted>,
    #[serde(rename = "service_account.updated")]
    pub service_account_updated: Option<crate::models::AuditLogServiceAccountUpdated>,
    #[serde(rename = "type")]
    pub _type: crate::models::AuditLogEventType,
    #[serde(rename = "user.added")]
    pub user_added: Option<crate::models::AuditLogUserAdded>,
    #[serde(rename = "user.deleted")]
    pub user_deleted: Option<crate::models::AuditLogUserDeleted>,
    #[serde(rename = "user.updated")]
    pub user_updated: Option<crate::models::AuditLogUserUpdated>,
}
