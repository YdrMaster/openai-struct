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
pub struct VectorStoreSearchResultItem {
    #[serde(rename = "attributes")]
    pub attributes: crate::models::VectorStoreFileAttributes,
    /// Content chunks from the file.
    #[serde(rename = "content")]
    pub content: Vec<crate::models::VectorStoreSearchResultContentObject>,
    /// The ID of the vector store file.
    #[serde(rename = "file_id")]
    pub file_id: String,
    /// The name of the vector store file.
    #[serde(rename = "filename")]
    pub filename: String,
    /// The similarity score for the result.
    #[serde(rename = "score")]
    pub score: f32,
}
