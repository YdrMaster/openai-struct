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
pub struct CreateAssistantRequestToolResourcesFileSearch {
    /// The [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Option<Vec<String>>,
    /// A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this assistant. There can be a maximum of 1 vector store attached to the assistant.
    #[serde(rename = "vector_stores")]
    pub vector_stores:
        Option<Vec<crate::models::CreateAssistantRequestToolResourcesFileSearchVectorStores>>,
}
