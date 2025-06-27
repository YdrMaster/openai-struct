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
pub struct FineTuningCheckpointPermission {
    /// The Unix timestamp (in seconds) for when the permission was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The permission identifier, which can be referenced in the API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always \"checkpoint.permission\".
    #[serde(rename = "object")]
    pub object: String,
    /// The project identifier that the permission is for.
    #[serde(rename = "project_id")]
    pub project_id: String,
}
