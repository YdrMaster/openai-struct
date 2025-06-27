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
pub struct ProjectServiceAccountCreateResponse {
    #[serde(rename = "api_key")]
    pub api_key: crate::models::ProjectServiceAccountApiKey,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "object")]
    pub object: String,
    /// Service accounts can only have one role of type `member`
    #[serde(rename = "role")]
    pub role: String,
}
