/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// AdminApiKey:
///   type: object
///   description: Represents an individual Admin API key in an org.
///   properties:
///     object:
///       type: string
///       example: organization.admin_api_key
///       description: The object type, which is always `organization.admin_api_key`
///       x-stainless-const: true
///     id:
///       type: string
///       example: key_abc
///       description: The identifier, which can be referenced in API endpoints
///     name:
///       type: string
///       example: Administration Key
///       description: The name of the API key
///     redacted_value:
///       type: string
///       example: sk-admin...def
///       description: The redacted value of the API key
///     value:
///       type: string
///       example: sk-admin-1234abcd
///       description: The value of the API key. Only shown on create.
///     created_at:
///       type: integer
///       format: int64
///       example: 1711471533
///       description: The Unix timestamp (in seconds) of when the API key was created
///     last_used_at:
///       type: integer
///       format: int64
///       nullable: true
///       example: 1711471534
///       description: The Unix timestamp (in seconds) of when the API key was last used
///     owner:
///       type: object
///       properties:
///         type:
///           type: string
///           example: user
///           description: Always `user`
///         object:
///           type: string
///           example: organization.user
///           description: The object type, which is always organization.user
///         id:
///           type: string
///           example: sa_456
///           description: The identifier, which can be referenced in API endpoints
///         name:
///           type: string
///           example: My Service Account
///           description: The name of the user
///         created_at:
///           type: integer
///           format: int64
///           example: 1711471533
///           description: The Unix timestamp (in seconds) of when the user was created
///         role:
///           type: string
///           example: owner
///           description: Always `owner`
///   required:
///     - object
///     - redacted_value
///     - name
///     - created_at
///     - last_used_at
///     - id
///     - owner
///   x-oaiMeta:
///     name: The admin API key object
///     example: |
///       {
///         "object": "organization.admin_api_key",
///         "id": "key_abc",
///         "name": "Main Admin Key",
///         "redacted_value": "sk-admin...xyz",
///         "created_at": 1711471533,
///         "last_used_at": 1711471534,
///         "owner": {
///           "type": "user",
///           "object": "organization.user",
///           "id": "user_123",
///           "name": "John Doe",
///           "created_at": 1711471533,
///           "role": "owner"
///         }
///       }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminApiKey {
    /// The Unix timestamp (in seconds) of when the API key was created
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: String,
    /// The Unix timestamp (in seconds) of when the API key was last used
    #[serde(rename = "last_used_at")]
    pub last_used_at: i64,
    /// The name of the API key
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `organization.admin_api_key`
    #[serde(default = "default_object")]
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "owner")]
    pub owner: crate::models::AdminApiKeyOwner,
    /// The redacted value of the API key
    #[serde(rename = "redacted_value")]
    pub redacted_value: String,
    /// The value of the API key. Only shown on create.
    #[serde(rename = "value")]
    pub value: Option<String>,
}

fn default_object() -> String {
    "organization.admin_api_key".into()
}
