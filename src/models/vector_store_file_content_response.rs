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
pub struct VectorStoreFileContentResponse {
    /// Parsed content of the file.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::VectorStoreFileContentResponseData>,
    /// Indicates if there are more content pages to fetch.
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// The token for the next page, if any.
    #[serde(rename = "next_page")]
    pub next_page: String,
    /// The object type, which is always `vector_store.file_content.page`
    #[serde(rename = "object")]
    pub object: String,
}
