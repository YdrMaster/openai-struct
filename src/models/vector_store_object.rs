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
pub struct VectorStoreObject {
    /// The Unix timestamp (in seconds) for when the vector store was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "expires_after")]
    pub expires_after: Option<crate::models::VectorStoreExpirationAfter>,
    /// The Unix timestamp (in seconds) for when the vector store will expire.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<i32>,
    #[serde(rename = "file_counts")]
    pub file_counts: crate::models::VectorStoreObjectFileCounts,
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The Unix timestamp (in seconds) for when the vector store was last active.
    #[serde(rename = "last_active_at")]
    pub last_active_at: i32,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// The name of the vector store.
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `vector_store`.
    #[serde(rename = "object")]
    pub object: String,
    /// The status of the vector store, which can be either `expired`, `in_progress`, or `completed`. A status of `completed` indicates that the vector store is ready for use.
    #[serde(rename = "status")]
    pub status: String,
    /// The total number of bytes used by the files in the vector store.
    #[serde(rename = "usage_bytes")]
    pub usage_bytes: i32,
}
