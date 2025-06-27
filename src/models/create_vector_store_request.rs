/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVectorStoreRequest {
    /// The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty.
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy: Option<Value>,
    #[serde(rename = "expires_after")]
    pub expires_after: Option<crate::models::VectorStoreExpirationAfter>,
    /// A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files.
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    /// The name of the vector store.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
