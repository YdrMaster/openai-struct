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
pub struct VectorStoreObjectFileCounts {
    /// The number of files that were cancelled.
    #[serde(rename = "cancelled")]
    pub cancelled: i32,
    /// The number of files that have been successfully processed.
    #[serde(rename = "completed")]
    pub completed: i32,
    /// The number of files that have failed to process.
    #[serde(rename = "failed")]
    pub failed: i32,
    /// The number of files that are currently being processed.
    #[serde(rename = "in_progress")]
    pub in_progress: i32,
    /// The total number of files.
    #[serde(rename = "total")]
    pub total: i32,
}
