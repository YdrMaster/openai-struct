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
pub struct InviteRequestProjects {
    /// Project's public ID
    #[serde(rename = "id")]
    pub id: String,
    /// Project membership role
    #[serde(rename = "role")]
    pub role: String,
}
