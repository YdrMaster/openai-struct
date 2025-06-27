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
pub struct VectorStoreFileBatchObject {
    /// The Unix timestamp (in seconds) for when the vector store files batch was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "file_counts")]
    pub file_counts: crate::models::VectorStoreFileBatchObjectFileCounts,
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always `vector_store.file_batch`.
    #[serde(rename = "object")]
    pub object: String,
    /// The status of the vector store files batch, which can be either `in_progress`, `completed`, `cancelled` or `failed`.
    #[serde(rename = "status")]
    pub status: String,
    /// The ID of the [vector store](/docs/api-reference/vector-stores/object) that the [File](/docs/api-reference/files) is attached to.
    #[serde(rename = "vector_store_id")]
    pub vector_store_id: String,
}
