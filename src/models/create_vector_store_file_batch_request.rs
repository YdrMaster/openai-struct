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
pub struct CreateVectorStoreFileBatchRequest {
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::models::VectorStoreFileAttributes>,
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy: Option<crate::models::ChunkingStrategyRequestParam>,
    /// A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files.
    #[serde(rename = "file_ids")]
    pub file_ids: Vec<String>,
}
