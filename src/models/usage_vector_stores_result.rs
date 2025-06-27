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
pub struct UsageVectorStoresResult {
    #[serde(rename = "object")]
    pub object: String,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    /// The vector stores usage in bytes.
    #[serde(rename = "usage_bytes")]
    pub usage_bytes: i32,
}
