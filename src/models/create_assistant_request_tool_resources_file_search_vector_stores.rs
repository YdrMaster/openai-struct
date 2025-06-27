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
pub struct CreateAssistantRequestToolResourcesFileSearchVectorStores {
    /// The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy.
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy: Option<Value>,
    /// A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store.
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
}
