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
pub struct BatchRequestCounts {
    /// Number of requests that have been completed successfully.
    #[serde(rename = "completed")]
    pub completed: i32,
    /// Number of requests that have failed.
    #[serde(rename = "failed")]
    pub failed: i32,
    /// Total number of requests in the batch.
    #[serde(rename = "total")]
    pub total: i32,
}
