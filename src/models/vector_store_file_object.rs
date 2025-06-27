/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub VectorStoreFileObject : A list of files attached to a vector store.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorStoreFileObject {
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::models::VectorStoreFileAttributes>,
    /// The strategy used to chunk the file.
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy: Option<Value>,
    /// The Unix timestamp (in seconds) for when the vector store file was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_error")]
    pub last_error: crate::models::VectorStoreFileObjectLastError,
    /// The object type, which is always `vector_store.file`.
    #[serde(rename = "object")]
    pub object: String,
    /// The status of the vector store file, which can be either `in_progress`, `completed`, `cancelled`, or `failed`. The status `completed` indicates that the vector store file is ready for use.
    #[serde(rename = "status")]
    pub status: String,
    /// The total vector store usage in bytes. Note that this may be different from the original file size.
    #[serde(rename = "usage_bytes")]
    pub usage_bytes: i32,
    /// The ID of the [vector store](/docs/api-reference/vector-stores/object) that the [File](/docs/api-reference/files) is attached to.
    #[serde(rename = "vector_store_id")]
    pub vector_store_id: String,
}
