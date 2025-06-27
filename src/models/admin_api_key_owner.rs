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
///
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminApiKeyOwner {
    /// The Unix timestamp (in seconds) of when the user was created
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The name of the user
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The object type, which is always organization.user
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: Option<String>,
    /// Always `owner`
    #[serde(rename = "role")]
    pub role: Option<String>,
    /// Always `user`
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

fn default_object() -> Option<String> {
    Some("organization.user".into())
}
